use std::fmt::Display;
use std::io::Read;
use std::string::FromUtf8Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SExprError {
    #[error("io error at {position}: {source}")]
    Io {
        #[source]
        source: std::io::Error,
        position: usize,
    },
    #[error("utf8 error at {position}: {source}")]
    Utf8 {
        #[source]
        source: FromUtf8Error,
        position: usize,
    },
    #[error("unexpected byte at {position}: found {unexpected:#x}, expected {expected:#x}")]
    UnexpectedByte {
        unexpected: u8,
        expected: u8,
        position: usize,
    },
    #[error("{context}: {source}")]
    WithContext {
        #[source]
        source: Box<SExprError>,
        context: String,
    },
}

impl SExprError {
    #[must_use]
    pub fn with_context(self, context: &str) -> SExprError {
        SExprError::WithContext {
            source: Box::new(self),
            context: context.to_string(),
        }
    }
}

fn read_one_byte(r: &mut crate::reader::Reader) -> Result<u8, SExprError> {
    let mut byte = [0u8; 1];
    let n = r.read(&mut byte).map_err(|e| SExprError::Io {
        source: e,
        position: r.position(),
    })?;
    if n == 0 {
        Err(SExprError::Io {
            source: std::io::Error::from(std::io::ErrorKind::UnexpectedEof),
            position: r.position(),
        })
    } else {
        Ok(byte[0])
    }
}

fn read_symbol(r: &mut crate::reader::Reader) -> Result<String, SExprError> {
    let mut buf = Vec::new();
    loop {
        if let Some(c) = r.peek()
            && !c.is_ascii_whitespace()
            && c != b'('
            && c != b')'
        {
            buf.push(c);
            r.advance(1).map_err(|e| SExprError::Io {
                source: e,
                position: r.position(),
            })?;
        } else {
            break;
        }
    }
    String::from_utf8(buf).map_err(|e| SExprError::Utf8 {
        source: e,
        position: r.position(),
    })
}

fn read_required_byte(r: &mut crate::reader::Reader, expected: u8) -> Result<(), SExprError> {
    let byte = read_one_byte(r)?;
    if byte == expected {
        Ok(())
    } else {
        Err(SExprError::UnexpectedByte {
            unexpected: byte,
            expected,
            position: r.position(),
        })
    }
}

fn read_text(r: &mut crate::reader::Reader) -> Result<String, SExprError> {
    let mut buf = Vec::new();
    // Check first item is quote
    read_required_byte(r, b'"').map_err(|err| err.with_context("parsing beginning of text"))?;
    loop {
        let escaped = match r.peek() {
            Some(b'\\') => {
                // Escape sequence
                r.advance(1).map_err(|err| SExprError::Io {
                    source: err,
                    position: r.position(),
                })?;
                true
            }
            Some(_)
           | None // EOF - we'll handle with the read_one_byte call
           => false,
        };
        let byte = read_one_byte(r)?;
        if !escaped && byte == b'"' {
            break;
        }
        buf.push(byte);
    }
    String::from_utf8(buf).map_err(|e| SExprError::Utf8 {
        source: e,
        position: r.position(),
    })
}

#[derive(Clone, Debug, PartialEq)]
pub enum SExprItem {
    Atom(String),
    Text(String),
    Node(String, Vec<SExprItem>),
}

impl Display for SExprItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExprItem::Atom(s) => write!(f, "{s}"),
            SExprItem::Text(s) => write!(f, "\"{s}\""),
            SExprItem::Node(name, items) => {
                write!(f, "({name}")?;
                for item in items {
                    write!(f, " {item}")?;
                }
                write!(f, ")")
            }
        }
    }
}

fn read_node(reader: &mut crate::reader::Reader) -> Result<SExprItem, SExprError> {
    // Read opening '('
    read_required_byte(reader, b'(')
        .map_err(|err| err.with_context("parsing beginning of new node"))?;
    // Read symbol
    let name = read_symbol(reader)?;
    let mut items = Vec::new();
    loop {
        reader.consume_whitespace().map_err(|err| SExprError::Io {
            source: err,
            position: reader.position(),
        })?;
        match reader.peek() {
            None => {
                return Err(SExprError::Io {
                    source: std::io::Error::from(std::io::ErrorKind::UnexpectedEof),
                    position: reader.position(),
                });
            }
            Some(b')') => {
                reader.advance(1).map_err(|err| SExprError::Io {
                    source: err,
                    position: reader.position(),
                })?;
                break;
            }
            Some(b'(') => {
                items.push(read_node(reader)?);
            }
            Some(b'"') => {
                items.push(SExprItem::Text(read_text(reader)?));
            }
            _ => {
                items.push(SExprItem::Atom(read_symbol(reader)?));
            }
        }
    }
    Ok(SExprItem::Node(name, items))
}

pub(crate) fn parse_sexpr_stream(input: &str) -> Result<Vec<SExprItem>, SExprError> {
    let mut reader = crate::reader::Reader::new(input);
    let mut out = Vec::new();
    loop {
        reader.consume_whitespace().map_err(|err| SExprError::Io {
            source: err,
            position: reader.position(),
        })?;
        if reader.is_eof() {
            break;
        }
        out.push(read_node(&mut reader)?);
    }

    Ok(out)
}

#[cfg(test)]
mod test {
    use crate::{reader::Reader, sexpr::*};

    #[test]
    fn test_sexpr_parse_escaped_text() {
        let input = r#""\"he\\l\\lo\\\"""#;
        let parsed = match read_text(&mut Reader::new(input)) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(parsed, r#""he\l\lo\""#);
    }

    #[test]
    fn test_sexpr_parse() {
        let input = r#"(typ "m" (inst (alias nat)))"#;
        let parsed = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![SExprItem::Node(
                "typ".to_string(),
                vec![
                    SExprItem::Text("m".to_string()),
                    SExprItem::Node(
                        "inst".to_string(),
                        vec![SExprItem::Node(
                            "alias".to_string(),
                            vec![SExprItem::Atom("nat".to_string())]
                        )]
                    )
                ]
            )]
        );
    }
}

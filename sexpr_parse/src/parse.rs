use crate::error::SExprError;
use crate::reader::Reader;
use crate::sexpr::SExprItem;
use std::io::Read;

fn read_one_byte(r: &mut Reader) -> Result<u8, SExprError> {
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

fn read_symbol(r: &mut Reader) -> Result<String, SExprError> {
    let mut buf = Vec::new();
    loop {
        if let Some(c) = r.peek()
            && !c.is_ascii_whitespace()
            && c != b'('
            && c != b')'
            && c != b'"'
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

fn ensure_item_boundary(r: &Reader) -> Result<(), SExprError> {
    match r.peek() {
        None | Some(b')') => Ok(()),
        Some(c) if c.is_ascii_whitespace() => Ok(()),
        Some(unexpected) => Err(SExprError::MissingSeparator {
            unexpected,
            position: r.position(),
        }),
    }
}

fn read_required_byte(r: &mut Reader, expected: u8) -> Result<(), SExprError> {
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

fn read_text(r: &mut Reader) -> Result<String, SExprError> {
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

fn read_node(reader: &mut Reader) -> Result<SExprItem, SExprError> {
    // Read opening '('
    read_required_byte(reader, b'(')
        .map_err(|err| err.with_context("parsing beginning of new node"))?;
    // Read symbol
    let name = read_symbol(reader)?;
    if name.is_empty() {
        return Err(SExprError::ExpectedSymbol {
            position: reader.position(),
        });
    }
    ensure_item_boundary(reader)?;
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
                ensure_item_boundary(reader)?;
            }
            Some(b'"') => {
                items.push(SExprItem::Text(read_text(reader)?));
                ensure_item_boundary(reader)?;
            }
            _ => {
                items.push(SExprItem::Atom(read_symbol(reader)?));
                ensure_item_boundary(reader)?;
            }
        }
    }
    Ok(SExprItem::Node(name, items))
}

pub fn parse_sexpr_stream(input: &str) -> Result<Vec<SExprItem>, SExprError> {
    let mut reader = Reader::new(input);
    let mut out = Vec::new();
    loop {
        reader.consume_whitespace().map_err(|err| SExprError::Io {
            source: err,
            position: reader.position(),
        })?;
        if reader.is_eof() {
            break;
        }
        let item = match reader.peek() {
            Some(b'(') => read_node(&mut reader),
            Some(b'"') => Ok(SExprItem::Text(read_text(&mut reader)?)),
            Some(_) => Ok(SExprItem::Atom(read_symbol(&mut reader)?)),
            None => Err(SExprError::Io {
                source: std::io::Error::from(std::io::ErrorKind::UnexpectedEof),
                position: reader.position(),
            }),
        }?;
        out.push(item);
        ensure_item_boundary(&reader)?;
    }

    Ok(out)
}

#[cfg(test)]
mod test {
    use crate::parse::*;

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

    #[test]
    fn test_parse_top_level_atom_and_text_items() {
        let input = "foo \"bar\"";
        let parsed = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![
                SExprItem::Atom("foo".to_string()),
                SExprItem::Text("bar".to_string())
            ]
        );
    }

    #[test]
    fn test_rejects_node_with_empty_name() {
        for input in ["()", "( )"] {
            assert!(
                parse_sexpr_stream(input).is_err(),
                "expected malformed node to be rejected: {input:?}"
            );
        }
    }

    #[test]
    fn test_rejects_missing_separator_between_node_name_and_text() {
        let input = r#"(typ"m")"#;
        assert!(
            parse_sexpr_stream(input).is_err(),
            "expected malformed node to be rejected: {input:?}"
        );
    }
}

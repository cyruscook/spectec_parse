#![deny(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![allow(clippy::doc_markdown)]

mod decode;
mod reader;
mod sexpr;
pub mod spectec;

pub use decode::DecodeError;
pub use sexpr::SExprError;

/// Parses a SpecTec AST stream from the input string.
///
/// # Errors
///
/// Will return an error if any of the s-expressions cannot be decoded, or if the s-expressions are
/// not a valid SpecTec AST stream.
pub fn parse_spectec_stream(input: &str) -> Result<Vec<spectec::SpecTecDef>, decode::DecodeError> {
    let sexpr_items = crate::sexpr::parse_sexpr_stream(input)?;
    let mut parsed = Vec::new();
    for item in sexpr_items {
        parsed.push(decode::Decode::decode(item)?);
    }
    Ok(parsed)
}

#[cfg(test)]
mod test {
    use crate::parse_spectec_stream;
    use crate::spectec::*;
    #[test]
    fn test_parse_spectec_stream() {
        let input = r#"
(typ "m" (inst))
(typ "n" (inst))
"#;
        let parsed = match parse_spectec_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![
                SpecTecDef::Typ {
                    id: "m".to_string(),
                    insts: vec![SpecTestInst::Inst {
                        bindings: vec![],
                        args: vec![],
                        deftyps: vec![],
                    }],
                    params: vec![],
                },
                SpecTecDef::Typ {
                    id: "n".to_string(),
                    insts: vec![SpecTestInst::Inst {
                        bindings: vec![],
                        args: vec![],
                        deftyps: vec![],
                    }],
                    params: vec![],
                },
            ]
        );
    }
}

#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]

mod decode;
mod reader;
mod sexpr;
pub mod spectec;

pub use decode::DecodeError;
pub use sexpr::SExprError;

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
                    insts: vec![SpecTestInst::Inst],
                    params: vec![],
                },
                SpecTecDef::Typ {
                    id: "n".to_string(),
                    insts: vec![SpecTestInst::Inst],
                    params: vec![],
                },
            ]
        );
    }
}

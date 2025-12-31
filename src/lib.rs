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
pub mod wasm;

pub use decode::DecodeError;
pub use sexpr::SExprError;

pub use spectec::parse_spectec_stream;
pub use wasm::get_wasm_spectec_ast;

#[cfg(test)]
mod test {
    use crate::parse_spectec_stream;
    use crate::spectec::*;
    #[test]
    fn test_parse_spectec_stream() {
        let input = r#"
(typ "m" (inst (alias nat)))
(typ "n" (inst (alias nat)))
"#;
        let parsed = match parse_spectec_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![
                SpecTecDef::Typ {
                    x: "m".to_string(),
                    insts: vec![SpecTecInst::Inst {
                        bs: vec![],
                        as_: vec![],
                        dt: SpecTecDefTyp::Alias {
                            typ: SpecTecTyp::Num(SpecTecNumTyp::Nat),
                        },
                    }],
                    ps: vec![],
                },
                SpecTecDef::Typ {
                    x: "n".to_string(),
                    insts: vec![SpecTecInst::Inst {
                        bs: vec![],
                        as_: vec![],
                        dt: SpecTecDefTyp::Alias {
                            typ: SpecTecTyp::Num(SpecTecNumTyp::Nat),
                        },
                    }],
                    ps: vec![],
                },
            ]
        );
    }
}

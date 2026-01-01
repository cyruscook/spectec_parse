#![deny(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

mod definitions;
mod error;
mod expressions;
mod grammars;
mod iterations;
mod literal;
mod nativetypes;
mod operators;
mod premises;
mod types;

pub use crate::{
    definitions::*, error::*, expressions::*, grammars::*, iterations::*, literal::*,
    nativetypes::*, operators::*, premises::*, types::*,
};

/// Parses a SpecTec AST stream from the input string.
///
/// # Errors
///
/// Will return an error if any of the s-expressions cannot be decoded, or if the s-expressions are
/// not a valid SpecTec AST stream.
pub fn parse_spectec_stream(input: &str) -> crate::Result<Vec<SpecTecDef>> {
    let sexpr_items = sexpr::parse_sexpr_stream(input)?;
    decode::Decode::decode(&mut sexpr_items.iter().peekable()).map_err(crate::Error::from)
}

#[cfg(test)]
mod test {
    use crate::*;
    use decode_derive::SExprDecode;
    use sexpr::parse_sexpr_stream;

    #[test]
    fn test_extra_string() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A { b: String },
        }

        let input = r#"(a "m" "c")"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed: decode::Result<TestEnum> =
            decode::Decode::decode(&mut sexprs.iter().peekable());
        assert!(parsed.is_err());
        assert_eq!(
            parsed.unwrap_err().to_string(),
            "Error decoding spectec_ast::test::test_extra_string::TestEnum::A: Extra unparsed S-expression remaining: Text(\"c\")"
        );
    }

    #[test]
    fn test_extra_item() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A { b: Vec<TestEnum> },
        }

        let input = r#"(a (a) (a) (b))"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed: decode::Result<TestEnum> =
            decode::Decode::decode(&mut sexprs.iter().peekable());
        dbg!(&parsed);
        assert!(parsed.is_err());
        assert_eq!(
            parsed.unwrap_err().to_string(),
            "Error decoding spectec_ast::test::test_extra_item::TestEnum::A: Extra unparsed S-expression remaining: Node(\"b\", [])"
        );
    }

    #[test]
    fn test_extra_item_unit() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A,
        }

        let input = r#"(a (b))"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed: decode::Result<TestEnum> =
            decode::Decode::decode(&mut sexprs.iter().peekable());
        assert!(parsed.is_err());
        assert_eq!(
            parsed.unwrap_err().to_string(),
            "Error decoding spectec_ast::test::test_extra_item_unit::TestEnum::A: Extra unparsed S-expression remaining: Node(\"b\", [])"
        );
    }

    #[test]
    fn test_incompat_item() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A { b: Vec<TestEnum> },
        }

        let input = r#"(a "a")"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed: decode::Result<TestEnum> =
            decode::Decode::decode(&mut sexprs.iter().peekable());
        assert!(parsed.is_err());
        assert_eq!(
            parsed.unwrap_err().to_string(),
            "Error decoding spectec_ast::test::test_incompat_item::TestEnum::A: Extra unparsed S-expression remaining: Text(\"a\")"
        );
    }

    #[test]
    fn test_spectec_vec() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A { b: Vec<TestEnum2> },
        }

        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum2 {
            #[sexpr_atom(name = "not")]
            Not,
            #[sexpr_atom(name = "plus")]
            Plus,
            #[sexpr_atom(name = "minus")]
            Minus,
            #[sexpr_atom(name = "plusminus")]
            PlusMinus,
            #[sexpr_atom(name = "minusplus")]
            MinusPlus,
        }

        let input = r#"(a minus minusplus not plus plusminus)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed: Vec<TestEnum> = match decode::Decode::decode(&mut sexprs.iter().peekable()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![TestEnum::A {
                b: vec![
                    TestEnum2::Minus,
                    TestEnum2::MinusPlus,
                    TestEnum2::Not,
                    TestEnum2::Plus,
                    TestEnum2::PlusMinus,
                ]
            }]
        );
    }

    #[test]
    fn test_spectec_atom_unnamed() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A { b: TestEnum2 },
        }
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum2 {
            #[sexpr_atom()]
            C(TestEnum3),
        }
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum3 {
            #[sexpr_atom(name = "d")]
            D,
        }

        let input = r#"(a d)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed: Vec<TestEnum> = match decode::Decode::decode(&mut sexprs.iter().peekable()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![TestEnum::A {
                b: TestEnum2::C(TestEnum3::D)
            }]
        );
    }

    #[test]
    fn test_spectec_node_unnamed_fields() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A(u64),
        }

        let input = r#"(a 0)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let parsed: Vec<TestEnum> = match decode::Decode::decode(&mut sexprs.iter().peekable()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(parsed, vec![TestEnum::A(0)]);
    }

    #[test]
    fn test_spectec_node_option_named_field() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A {
                b: Option<u64>,
                c: Option<u64>,
                d: Option<bool>,
            },
        }

        let input = r#"(a 0 false)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let parsed: Vec<TestEnum> = match decode::Decode::decode(&mut sexprs.iter().peekable()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![TestEnum::A {
                b: Some(0),
                c: None,
                d: Some(false)
            }]
        );
    }

    #[test]
    fn test_spectec_node_option_unnamed_field() {
        #[derive(SExprDecode, Debug, PartialEq)]
        pub enum TestEnum {
            #[sexpr_node(name = "a")]
            A(Option<u64>),
        }

        let input = r#"(a)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let parsed: Vec<TestEnum> = match decode::Decode::decode(&mut sexprs.iter().peekable()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(parsed, vec![TestEnum::A(None)]);
    }

    #[test]
    fn test_spectec_def_enum() {
        let input = r#"(typ "M" (inst (alias nat)))"#;
        let sexprs = match sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed: Vec<SpecTecDef> = match decode::Decode::decode(&mut sexprs.iter().peekable()) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        assert_eq!(
            parsed,
            vec![SpecTecDef::Typ {
                x: "M".to_string(),
                ps: vec![],
                insts: vec![SpecTecInst::Inst {
                    bs: vec![],
                    as_: vec![],
                    dt: SpecTecDefTyp::Alias {
                        typ: SpecTecTyp::Num(SpecTecNumTyp::Nat),
                    },
                }]
            }]
        );
    }

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

#![deny(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

mod definitions;
mod expressions;
mod grammars;
mod iterations;
mod literal;
mod nativetypes;
mod operators;
mod premises;
mod types;

pub use crate::{
    definitions::*, expressions::*, grammars::*, iterations::*, literal::*, nativetypes::*,
    operators::*, premises::*, types::*,
};

/// Parses a SpecTec AST stream from the input string.
///
/// # Errors
///
/// Will return an error if any of the s-expressions cannot be decoded, or if the s-expressions are
/// not a valid SpecTec AST stream.
pub fn parse_spectec_stream(input: &str) -> Result<Vec<SpecTecDef>, spectec_decode::DecodeError> {
    let sexpr_items = sexpr::parse_sexpr_stream(input)?;
    let mut parsed = Vec::new();
    for item in sexpr_items {
        parsed.push(spectec_decode::Decode::decode(item)?);
    }
    Ok(parsed)
}

#[cfg(test)]
mod test {
    use crate::*;
    use sexpr::parse_sexpr_stream;
    use spectec_decode::Decode;
    use spectec_derive::SpecTecItem;

    #[test]
    fn test_extra_string() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A { b: String },
        }

        let input = r#"(a "m" "c")"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_ast::test::test_extra_string::TestEnum: decoding variant A item not consumed by fields (A { b: \"m\" }): Unexpected item 'Text(\"c\")'"
            );
        }
    }

    #[test]
    fn test_extra_item() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<TestEnum>,
            },
        }

        let input = r#"(a (a) (a) (b))"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_ast::test::test_extra_item::TestEnum: decoding variant A item not consumed by fields (A { b: [A { b: [] }, A { b: [] }] }): Unexpected item 'Node(\"b\", [])'"
            );
        }
    }

    #[test]
    fn test_extra_item_unit() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A,
        }

        let input = r#"(a (b))"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_ast::test::test_extra_item_unit::TestEnum: decoding variant A unit type should have no items: Unexpected item 'Node(\"b\", [])'"
            );
        }
    }

    #[test]
    fn test_incompat_item() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<TestEnum>,
            },
        }

        let input = r#"(a "a")"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_ast::test::test_incompat_item::TestEnum: decoding variant A item not consumed by fields (A { b: [] }): Unexpected item 'Text(\"a\")'"
            );
        }
    }

    #[test]
    fn test_spectec_vec() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<TestEnum2>,
            },
        }

        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum2 {
            #[spectec_atom(name = "not")]
            Not,
            #[spectec_atom(name = "plus")]
            Plus,
            #[spectec_atom(name = "minus")]
            Minus,
            #[spectec_atom(name = "plusminus")]
            PlusMinus,
            #[spectec_atom(name = "minusplus")]
            MinusPlus,
        }

        let input = r#"(a minus minusplus not plus plusminus)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
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
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A { b: TestEnum2 },
        }
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum2 {
            #[spectec_atom()]
            C(TestEnum3),
        }
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum3 {
            #[spectec_atom(name = "d")]
            D,
        }

        let input = r#"(a d)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parsed,
            vec![TestEnum::A {
                b: TestEnum2::C(TestEnum3::D)
            }]
        );
    }

    #[test]
    fn test_spectec_node_unnamed_fields() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A(u64),
        }

        let input = r#"(a 0)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(parsed, vec![TestEnum::A(0)]);
    }

    #[test]
    fn test_spectec_node_option_named_field() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(option = true)]
                b: Option<u64>,
                #[spectec_field(option = true)]
                c: Option<u64>,
                #[spectec_field(option = true)]
                d: Option<bool>,
            },
        }

        let input = r#"(a 0 false)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
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
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A(#[spectec_field(option = true)] Option<u64>),
        }

        let input = r#"(a)"#;
        let sexprs = match parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(parsed, vec![TestEnum::A(None)]);
    }

    #[test]
    fn test_spectec_def_enum() {
        let input = r#"(typ "M" (inst (alias nat)))"#;
        let sexprs = match sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(SpecTecDef::can_decode(&sexpr));
                match SpecTecDef::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
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

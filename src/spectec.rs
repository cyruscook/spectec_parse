use spectec_derive::{SpecTecAtom, SpecTecNode};

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L216>
#[allow(unused)]
#[derive(SpecTecNode, Debug, PartialEq)]
pub enum SpecTecDef {
    #[spectec_node(name = "typ")]
    Typ {
        id: String,
        #[spectec_field(vec = true)]
        params: Vec<SpecTestParam>,
        #[spectec_field(vec = true)]
        insts: Vec<SpecTestInst>,
    },
    #[spectec_node(name = "rel")]
    Rel,
    #[spectec_node(name = "def")]
    Def,
    #[spectec_node(name = "gram")]
    Gram,
    #[spectec_node(name = "rec")]
    Rec,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L189>
#[allow(unused)]
#[derive(SpecTecNode, Debug, PartialEq)]
pub enum SpecTestParam {
    #[spectec_node(name = "exp")]
    Exp,
    #[spectec_node(name = "typ")]
    Typ,
    #[spectec_node(name = "def")]
    Def,
    #[spectec_node(name = "gram")]
    Gram,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L196>
#[allow(unused)]
#[derive(SpecTecNode, Debug, PartialEq)]
pub enum SpecTestInst {
    #[spectec_node(name = "inst")]
    Inst {
        #[spectec_field(vec = true)]
        bindings: Vec<SpecTestBind>,
        #[spectec_field(vec = true)]
        args: Vec<SpecTestArg>,
        #[spectec_field(vec = true)]
        deftyps: Vec<SpecTestArg>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#182>
#[allow(unused)]
#[derive(SpecTecNode, Debug, PartialEq)]
pub enum SpecTestBind {
    #[spectec_node(name = "exp")]
    Exp,
    #[spectec_node(name = "typ")]
    Typ,
    #[spectec_node(name = "def")]
    Def,
    #[spectec_node(name = "gram")]
    Gram,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#175>
#[allow(unused)]
#[derive(SpecTecNode, Debug, PartialEq)]
pub enum SpecTestArg {
    #[spectec_node(name = "exp")]
    Exp,
    #[spectec_node(name = "typ")]
    Typ,
    #[spectec_node(name = "def")]
    Def,
    #[spectec_node(name = "gram")]
    Gram,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#28>
#[allow(unused)]
#[derive(SpecTecAtom, Debug, PartialEq)]
pub enum SpecTecUnOp {
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

#[cfg(test)]
mod test {
    use crate::decode::Decode;
    use crate::spectec::*;

    #[test]
    fn test_extra_string() {
        #[derive(SpecTecNode, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A { b: String },
        }

        let input = r#"(a "m" "c")"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "Unexpected item 'Text(\"c\")'"
            );
        }
    }

    #[test]
    fn test_extra_item() {
        #[derive(SpecTecNode, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<TestEnum>,
            },
        }

        let input = r#"(a (a) (a) (b))"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "Unexpected item 'Node(\"b\", [])'"
            );
        }
    }

    #[test]
    fn test_extra_item_unit() {
        #[derive(SpecTecNode, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A,
        }

        let input = r#"(a (b))"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "Unexpected item 'Node(\"b\", [])'"
            );
        }
    }

    #[test]
    fn test_incompat_item() {
        #[derive(SpecTecNode, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<TestEnum>,
            },
        }

        let input = r#"(a "a")"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "Unexpected item 'Text(\"a\")'"
            );
        }
    }

    #[test]
    fn test_spectec_def_enum() {
        let input = r#"(typ "m" (inst))"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| match SpecTecDef::decode(sexpr) {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parsed,
            vec![SpecTecDef::Typ {
                id: "m".to_string(),
                params: vec![],
                insts: vec![SpecTestInst::Inst {
                    bindings: vec![],
                    args: vec![],
                    deftyps: vec![],
                }]
            }]
        );
    }

    #[test]
    fn test_spectec_atom() {
        #[derive(SpecTecNode, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<SpecTecUnOp>,
            },
        }

        let input = r#"(a minus minusplus not plus plusminus)"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| match TestEnum::decode(sexpr) {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parsed,
            vec![TestEnum::A {
                b: vec![
                    SpecTecUnOp::Minus,
                    SpecTecUnOp::MinusPlus,
                    SpecTecUnOp::Not,
                    SpecTecUnOp::Plus,
                    SpecTecUnOp::PlusMinus,
                ]
            }]
        );
    }
}

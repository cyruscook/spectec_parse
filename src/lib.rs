#![deny(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]

mod decode;
mod reader;
mod sexpr;
pub use decode::*;

pub use spectec_derive::*;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L216>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecDef {
    #[spectec_item(name = "typ")]
    Typ {
        id: String,
        #[spectec_field(vec = true)]
        params: Vec<SpecTestParam>,
        #[spectec_field(vec = true)]
        insts: Vec<SpecTestInst>,
    },
    #[spectec_item(name = "rel")]
    Rel,
    #[spectec_item(name = "def")]
    Def,
    #[spectec_item(name = "gram")]
    Gram,
    #[spectec_item(name = "rec")]
    Rec,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L189>
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTestParam {
    #[spectec_item(name = "exp")]
    Exp,
    #[spectec_item(name = "typ")]
    Typ,
    #[spectec_item(name = "def")]
    Def,
    #[spectec_item(name = "gram")]
    Gram,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L196>
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTestInst {
    #[spectec_item(name = "inst")]
    Inst,
}

#[cfg(test)]
mod test {
    use crate::SpecTecDef;
    use crate::decode::Decode;
    use spectec_derive::*;

    #[test]
    fn test_extra_string() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_item(name = "a")]
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
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_item(name = "a")]
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
    fn test_incompat_item() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_item(name = "a")]
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
        let input = r#"(typ "m" (inst (alias nat)))"#;
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
                insts: vec![crate::SpecTestInst::Inst]
            }]
        );
    }
}

mod decode;
mod reader;
mod sexpr;
pub use decode::*;

pub use spectec_derive::*;

/// https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L216
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecDef {
    #[spectec_item(name = "typ")]
    Typ, /* {
             id: String,
             params: Vec<SpecTestParam>,
             insts: Vec<SpecTestInst>,
         } */
    #[spectec_item(name = "rel")]
    Rel,
    #[spectec_item(name = "def")]
    Def,
    #[spectec_item(name = "gram")]
    Gram,
    #[spectec_item(name = "rec")]
    Rec,
}
/*
#[derive(SpecTecItem, Debug, PartialEq)]
enum SpecTestParam {}

#[derive(SpecTecItem, Debug, PartialEq)]
enum SpecTestInst {} */

#[cfg(test)]
mod test {
    use crate::SpecTecDef;
    use crate::decode::Decode;

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
        assert_eq!(parsed, vec![SpecTecDef::Typ]);
    }
}

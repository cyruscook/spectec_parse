use crate::SpecTecExp;
use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#58>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecIter {
    #[sexpr_atom(name = "opt")]
    Opt,
    #[sexpr_atom(name = "list")]
    List,
    #[sexpr_atom(name = "list1")]
    List1,
    #[sexpr_node(name = "listn")]
    ListN { e: Vec<SpecTecExp>, xo: Vec<String> },
}

use crate::{SpecTecArg, SpecTecExp, SpecTecIter, SpecTecIterExp};
use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L149>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecSym {
    #[sexpr_node(name = "var")]
    Var { x: String, as1: Vec<SpecTecArg> },
    #[sexpr_node(name = "num")]
    Num { n: i64 },
    #[sexpr_node(name = "text")]
    Text { t: String },
    #[sexpr_atom(name = "eps")]
    Eps,
    #[sexpr_node(name = "seq")]
    Seq { gs: Vec<SpecTecSym> },
    #[sexpr_node(name = "alt")]
    Alt { gs: Vec<SpecTecSym> },
    #[sexpr_node(name = "range")]
    Range {
        g1: Box<SpecTecSym>,
        g2: Box<SpecTecSym>,
    },
    #[sexpr_node(name = "iter")]
    Iter {
        g1: Box<SpecTecSym>,
        it: SpecTecIter,
        xes: Vec<SpecTecIterExp>,
    },
    #[sexpr_node(name = "attr")]
    Attr { e: SpecTecExp, g1: Box<SpecTecSym> },
}

use crate::{SpecTecArg, SpecTecExp, SpecTecIter, SpecTecIterExp};
use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L149>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecSym {
    #[spectec_node(name = "var")]
    Var {
        x: String,
        #[spectec_field(vec = true)]
        as1: Vec<SpecTecArg>,
    },
    #[spectec_node(name = "num")]
    Num { n: i64 },
    #[spectec_node(name = "text")]
    Text { t: String },
    #[spectec_atom(name = "eps")]
    Eps,
    #[spectec_node(name = "seq")]
    Seq {
        #[spectec_field(vec = true)]
        gs: Vec<SpecTecSym>,
    },
    #[spectec_node(name = "alt")]
    Alt {
        #[spectec_field(vec = true)]
        gs: Vec<SpecTecSym>,
    },
    #[spectec_node(name = "range")]
    Range {
        g1: Box<SpecTecSym>,
        g2: Box<SpecTecSym>,
    },
    #[spectec_node(name = "iter")]
    Iter {
        g1: Box<SpecTecSym>,
        it: SpecTecIter,
        #[spectec_field(vec = true)]
        xes: Vec<SpecTecIterExp>,
    },
    #[spectec_node(name = "attr")]
    Attr { e: SpecTecExp, g1: Box<SpecTecSym> },
}

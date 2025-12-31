use crate::{MixOp, SpecTecExp, SpecTecIter, SpecTecIterExp};
use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L164>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecPrem {
    #[spectec_node(name = "rule")]
    Rule { x: String, op: MixOp, e: SpecTecExp },
    #[spectec_node(name = "if")]
    If { e: SpecTecExp },
    #[spectec_node(name = "let")]
    Let { e1: SpecTecExp, e2: SpecTecExp },
    #[spectec_atom(name = "else")]
    Else,
    #[spectec_node(name = "iter")]
    Iter {
        pr1: Box<SpecTecPrem>,
        it: SpecTecIter,
        #[spectec_field(vec = true)]
        xes: Vec<SpecTecIterExp>,
    },
}

use crate::{MixOp, SpecTecExp, SpecTecIter, SpecTecIterExp};
use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L164>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecPrem {
    #[sexpr_node(name = "rule")]
    Rule { x: String, op: MixOp, e: SpecTecExp },
    #[sexpr_node(name = "if")]
    If { e: SpecTecExp },
    #[sexpr_node(name = "let")]
    Let { e1: SpecTecExp, e2: SpecTecExp },
    #[sexpr_atom(name = "else")]
    Else,
    #[sexpr_node(name = "iter")]
    Iter {
        pr1: Box<SpecTecPrem>,
        it: SpecTecIter,
        xes: Vec<SpecTecIterExp>,
    },
}

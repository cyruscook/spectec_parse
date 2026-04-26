use crate::{MixOp, SpecTecExp, SpecTecIter, SpecTecIterExp};
use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/d7b678327cd370cdbc5acfa94bd108772e2bef68/spectec/src/backend-ast/print.ml#L161>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecPrem {
    #[sexpr_node(name = "rule")]
    Rule {
        x: String,
        as1: Vec<crate::SpecTecArg>,
        op: MixOp,
        e: SpecTecExp,
    },
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

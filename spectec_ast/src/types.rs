use crate::{
    SpecTecArg, SpecTecBind, SpecTecBoolTyp, SpecTecExp, SpecTecIter, SpecTecNumTyp, SpecTecPrem,
};
use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L70>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecOpTyp {
    #[sexpr_atom()]
    Num(SpecTecNumTyp),
    #[sexpr_atom()]
    Bool(SpecTecBoolTyp),
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#74>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecTyp {
    #[sexpr_node(name = "var")]
    Var { x: String, as1: Vec<SpecTecArg> },
    #[sexpr_atom(name = "bool")]
    Bool,
    #[sexpr_atom()]
    Num(SpecTecNumTyp),
    #[sexpr_atom(name = "text")]
    Text,
    #[sexpr_node(name = "tup")]
    Tup { ets: Vec<SpecTecTypBind> },
    #[sexpr_node(name = "iter")]
    Iter {
        t1: Box<SpecTecTyp>,
        it: Vec<SpecTecIter>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#83>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecDefTyp {
    #[sexpr_node(name = "alias")]
    Alias { typ: SpecTecTyp },
    #[sexpr_node(name = "struct")]
    Struct { tfs: Vec<SpecTecTypField> },
    #[sexpr_node(name = "variant")]
    Variant { tcs: Vec<SpecTecTypCase> },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#89>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecTypBind {
    #[sexpr_node(name = "bind")]
    Bind { exp: SpecTecExp, typ: SpecTecTyp },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#92>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecTypField {
    #[sexpr_node(name = "field")]
    Field {
        at: crate::literal::MixOp,
        bs: Vec<SpecTecBind>,
        t: SpecTecTyp,
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#95>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecTypCase {
    #[sexpr_node(name = "case")]
    Field {
        op: crate::literal::MixOp,
        bs: Vec<SpecTecBind>,
        t: SpecTecTyp,
        prs: Vec<SpecTecPrem>,
    },
}

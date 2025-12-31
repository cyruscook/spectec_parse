use crate::{
    SpecTecArg, SpecTecBind, SpecTecBoolTyp, SpecTecExp, SpecTecIter, SpecTecNumTyp, SpecTecPrem,
};
use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L70>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecOpTyp {
    #[spectec_atom()]
    Num(SpecTecNumTyp),
    #[spectec_atom()]
    Bool(SpecTecBoolTyp),
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#74>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecTyp {
    #[spectec_node(name = "var")]
    Var {
        x: String,
        #[spectec_field(vec = true)]
        as1: Vec<SpecTecArg>,
    },
    #[spectec_atom(name = "bool")]
    Bool,
    #[spectec_atom()]
    Num(SpecTecNumTyp),
    #[spectec_atom(name = "text")]
    Text,
    #[spectec_node(name = "tup")]
    Tup {
        #[spectec_field(vec = true)]
        ets: Vec<SpecTecTypBind>,
    },
    #[spectec_node(name = "iter")]
    Iter {
        t1: Box<SpecTecTyp>,
        #[spectec_field(vec = true)]
        it: Vec<SpecTecIter>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#83>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecDefTyp {
    #[spectec_node(name = "alias")]
    Alias { typ: SpecTecTyp },
    #[spectec_node(name = "struct")]
    Struct {
        #[spectec_field(vec = true)]
        tfs: Vec<SpecTecTypField>,
    },
    #[spectec_node(name = "variant")]
    Variant {
        #[spectec_field(vec = true)]
        tcs: Vec<SpecTecTypCase>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#89>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecTypBind {
    #[spectec_node(name = "bind")]
    Bind { exp: SpecTecExp, typ: SpecTecTyp },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#92>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecTypField {
    #[spectec_node(name = "field")]
    Field {
        at: crate::literal::MixOp,
        #[spectec_field(vec = true)]
        bs: Vec<SpecTecBind>,
        t: SpecTecTyp,
        #[spectec_field(vec = true)]
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#95>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecTypCase {
    #[spectec_node(name = "case")]
    Field {
        op: crate::literal::MixOp,
        #[spectec_field(vec = true)]
        bs: Vec<SpecTecBind>,
        t: SpecTecTyp,
        #[spectec_field(vec = true)]
        prs: Vec<SpecTecPrem>,
    },
}

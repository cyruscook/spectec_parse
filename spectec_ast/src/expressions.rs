use crate::{
    MixOp, SpecTecArg, SpecTecBinOp, SpecTecCmpOp, SpecTecIter, SpecTecNum, SpecTecNumTyp,
    SpecTecOpTyp, SpecTecTyp, SpecTecUnOp,
};
use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#101>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecExp {
    #[sexpr_node(name = "var")]
    Var { id: String },
    #[sexpr_node(name = "bool")]
    Bool { b: bool },
    #[sexpr_node(name = "num")]
    Num { n: SpecTecNum },
    #[sexpr_node(name = "text")]
    Text { t: String },
    #[sexpr_node(name = "un")]
    Un {
        op: SpecTecUnOp,
        t: SpecTecOpTyp,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "bin")]
    Bin {
        op: SpecTecBinOp,
        t: SpecTecOpTyp,
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "cmp")]
    Cmp {
        op: SpecTecCmpOp,
        t: SpecTecOpTyp,
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "idx")]
    Idx {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "slice")]
    Slice {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
        e3: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "upd")]
    Upd {
        e1: Box<SpecTecExp>,
        path: Box<SpecTecPath>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "ext")]
    Ext {
        e1: Box<SpecTecExp>,
        path: Box<SpecTecPath>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "struct")]
    Str { efs: Vec<SpecTecExpField> },
    #[sexpr_node(name = "dot")]
    Dot { e1: Box<SpecTecExp>, at: MixOp },
    #[sexpr_node(name = "comp")]
    Comp {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "mem")]
    Mem {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "len")]
    Len { e1: Box<SpecTecExp> },
    #[sexpr_node(name = "tup")]
    Tup { es: Vec<SpecTecExp> },
    #[sexpr_node(name = "call")]
    Call { x: String, as1: Vec<SpecTecArg> },
    #[sexpr_node(name = "iter")]
    Iter {
        e1: Box<SpecTecExp>,
        it: SpecTecIter,
        xes: Vec<SpecTecIterExp>,
    },
    #[sexpr_node(name = "proj")]
    Proj { e1: Box<SpecTecExp>, i: i64 },
    #[sexpr_node(name = "case")]
    Case { op: MixOp, e1: Box<SpecTecExp> },
    #[sexpr_node(name = "uncase")]
    Uncase { e1: Box<SpecTecExp>, op: MixOp },
    #[sexpr_node(name = "opt")]
    Opt { eo: Option<Box<SpecTecExp>> },
    #[sexpr_node(name = "unopt")]
    Unopt { e1: Box<SpecTecExp> },
    #[sexpr_node(name = "list")]
    List { es: Vec<SpecTecExp> },
    #[sexpr_node(name = "lift")]
    Lift { e1: Box<SpecTecExp> },
    #[sexpr_node(name = "cat")]
    Cat {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "cvt")]
    Cvt {
        nt1: SpecTecNumTyp,
        nt2: SpecTecNumTyp,
        e1: Box<SpecTecExp>,
    },
    #[sexpr_node(name = "sub")]
    Sub {
        t1: SpecTecTyp,
        t2: SpecTecTyp,
        e1: Box<SpecTecExp>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#133>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecExpField {
    #[sexpr_node(name = "field")]
    Field { at: MixOp, e: SpecTecExp },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#136>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecPath {
    #[sexpr_atom(name = "root")]
    Root,
    #[sexpr_node(name = "idx")]
    Idx { p1: Box<SpecTecPath>, e: SpecTecExp },
    #[sexpr_node(name = "slice")]
    Slice {
        p1: Box<SpecTecPath>,
        e1: SpecTecExp,
        e2: SpecTecExp,
    },
    #[sexpr_node(name = "dot")]
    Dot { p1: Box<SpecTecPath>, at: MixOp },
}

// Usage of this type must be preceded with a `crate::spectec::iterations::SpecTecIter` value, not included here
/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#143>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecIterExp {
    #[sexpr_node(name = "dom")]
    Dom { x: String, e: SpecTecExp },
}

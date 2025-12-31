use crate::{
    MixOp, SpecTecArg, SpecTecBinOp, SpecTecCmpOp, SpecTecIter, SpecTecNum, SpecTecNumTyp,
    SpecTecOpTyp, SpecTecTyp, SpecTecUnOp,
};
use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#101>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecExp {
    #[spectec_node(name = "var")]
    Var { id: String },
    #[spectec_node(name = "bool")]
    Bool { b: bool },
    #[spectec_node(name = "num")]
    Num { n: SpecTecNum },
    #[spectec_node(name = "text")]
    Text { t: String },
    #[spectec_node(name = "un")]
    Un {
        op: SpecTecUnOp,
        t: SpecTecOpTyp,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "bin")]
    Bin {
        op: SpecTecBinOp,
        t: SpecTecOpTyp,
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "cmp")]
    Cmp {
        op: SpecTecCmpOp,
        t: SpecTecOpTyp,
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "idx")]
    Idx {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "slice")]
    Slice {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
        e3: Box<SpecTecExp>,
    },
    #[spectec_node(name = "upd")]
    Upd {
        e1: Box<SpecTecExp>,
        path: Box<SpecTecPath>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "ext")]
    Ext {
        e1: Box<SpecTecExp>,
        path: Box<SpecTecPath>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "struct")]
    Str {
        #[spectec_field(vec = true)]
        efs: Vec<SpecTecExpField>,
    },
    #[spectec_node(name = "dot")]
    Dot { e1: Box<SpecTecExp>, at: MixOp },
    #[spectec_node(name = "comp")]
    Comp {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "mem")]
    Mem {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "len")]
    Len { e1: Box<SpecTecExp> },
    #[spectec_node(name = "tup")]
    Tup {
        #[spectec_field(vec = true)]
        es: Vec<SpecTecExp>,
    },
    #[spectec_node(name = "call")]
    Call {
        x: String,
        #[spectec_field(vec = true)]
        as1: Vec<SpecTecArg>,
    },
    #[spectec_node(name = "iter")]
    Iter {
        e1: Box<SpecTecExp>,
        it: SpecTecIter,
        #[spectec_field(vec = true)]
        xes: Vec<SpecTecIterExp>,
    },
    #[spectec_node(name = "proj")]
    Proj { e1: Box<SpecTecExp>, i: i64 },
    #[spectec_node(name = "case")]
    Case { op: MixOp, e1: Box<SpecTecExp> },
    #[spectec_node(name = "uncase")]
    Uncase { e1: Box<SpecTecExp>, op: MixOp },
    #[spectec_node(name = "opt")]
    Opt {
        #[spectec_field(option = true)]
        eo: Option<Box<SpecTecExp>>,
    },
    #[spectec_node(name = "unopt")]
    Unopt { e1: Box<SpecTecExp> },
    #[spectec_node(name = "list")]
    List {
        #[spectec_field(vec = true)]
        es: Vec<SpecTecExp>,
    },
    #[spectec_node(name = "lift")]
    Lift { e1: Box<SpecTecExp> },
    #[spectec_node(name = "cat")]
    Cat {
        e1: Box<SpecTecExp>,
        e2: Box<SpecTecExp>,
    },
    #[spectec_node(name = "cvt")]
    Cvt {
        nt1: SpecTecNumTyp,
        nt2: SpecTecNumTyp,
        e1: Box<SpecTecExp>,
    },
    #[spectec_node(name = "sub")]
    Sub {
        t1: SpecTecTyp,
        t2: SpecTecTyp,
        e1: Box<SpecTecExp>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#133>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecExpField {
    #[spectec_node(name = "field")]
    Field { at: MixOp, e: SpecTecExp },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#136>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecPath {
    #[spectec_atom(name = "root")]
    Root,
    #[spectec_node(name = "idx")]
    Idx { p1: Box<SpecTecPath>, e: SpecTecExp },
    #[spectec_node(name = "slice")]
    Slice {
        p1: Box<SpecTecPath>,
        e1: SpecTecExp,
        e2: SpecTecExp,
    },
    #[spectec_node(name = "dot")]
    Dot { p1: Box<SpecTecPath>, at: MixOp },
}

// Usage of this type must be preceded with a `crate::spectec::iterations::SpecTecIter` value, not included here
/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#143>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecIterExp {
    #[spectec_node(name = "dom")]
    Dom { x: String, e: SpecTecExp },
}

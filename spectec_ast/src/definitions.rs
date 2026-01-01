use crate::{MixOp, SpecTecDefTyp, SpecTecExp, SpecTecPrem, SpecTecSym, SpecTecTyp};
use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L175>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecArg {
    #[sexpr_node(name = "exp")]
    Exp { e: SpecTecExp },
    #[sexpr_node(name = "typ")]
    Typ { t: SpecTecTyp },
    #[sexpr_node(name = "def")]
    Def { x: String },
    #[sexpr_node(name = "gram")]
    Gram { g: SpecTecSym },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L182>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecBind {
    #[sexpr_node(name = "exp")]
    Exp { x: String, t: SpecTecTyp },
    #[sexpr_node(name = "typ")]
    Typ { x: String },
    #[sexpr_node(name = "def")]
    Def {
        x: String,
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
    },
    #[sexpr_node(name = "gram")]
    Gram {
        x: String,
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L189>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecParam {
    #[sexpr_node(name = "exp")]
    Exp { x: String, t: SpecTecTyp },
    #[sexpr_node(name = "typ")]
    Typ { x: String },
    #[sexpr_node(name = "def")]
    Def {
        x: String,
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
    },
    #[sexpr_node(name = "gram")]
    Gram { x: String, t: SpecTecTyp },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L196>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecInst {
    #[sexpr_node(name = "inst")]
    Inst {
        bs: Vec<SpecTecBind>,
        as_: Vec<SpecTecArg>,
        dt: SpecTecDefTyp,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L201>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecRule {
    #[sexpr_node(name = "rule")]
    Rule {
        x: String,
        bs: Vec<SpecTecBind>,
        op: MixOp,
        e: SpecTecExp,
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L206>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecClause {
    #[sexpr_node(name = "clause")]
    Clause {
        bs: Vec<SpecTecBind>,
        as_: Vec<SpecTecArg>,
        e: SpecTecExp,
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L211>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecProd {
    #[sexpr_node(name = "prod")]
    Prod {
        bs: Vec<SpecTecBind>,
        g: SpecTecSym,
        e: SpecTecExp,
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L216>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecDef {
    #[sexpr_node(name = "typ")]
    Typ {
        x: String,
        ps: Vec<SpecTecParam>,
        insts: Vec<SpecTecInst>,
    },
    #[sexpr_node(name = "rel")]
    Rel {
        x: String,
        op: MixOp,
        t: SpecTecTyp,
        rules: Vec<SpecTecRule>,
    },
    #[sexpr_node(name = "def")]
    Dec {
        x: String,
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
        clauses: Vec<SpecTecClause>,
    },
    #[sexpr_node(name = "gram")]
    Gram {
        x: String,
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
        prods: Vec<SpecTecProd>,
    },
    #[sexpr_node(name = "rec")]
    Rec { ds: Vec<SpecTecDef> },
}

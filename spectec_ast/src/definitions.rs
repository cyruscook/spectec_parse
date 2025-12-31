use crate::{MixOp, SpecTecDefTyp, SpecTecExp, SpecTecPrem, SpecTecSym, SpecTecTyp};
use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L175>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecArg {
    #[spectec_node(name = "exp")]
    Exp { e: SpecTecExp },
    #[spectec_node(name = "typ")]
    Typ { t: SpecTecTyp },
    #[spectec_node(name = "def")]
    Def { x: String },
    #[spectec_node(name = "gram")]
    Gram { g: SpecTecSym },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L182>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecBind {
    #[spectec_node(name = "exp")]
    Exp { x: String, t: SpecTecTyp },
    #[spectec_node(name = "typ")]
    Typ { x: String },
    #[spectec_node(name = "def")]
    Def {
        x: String,
        #[spectec_field(vec = true)]
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
    },
    #[spectec_node(name = "gram")]
    Gram {
        x: String,
        #[spectec_field(vec = true)]
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L189>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecParam {
    #[spectec_node(name = "exp")]
    Exp { x: String, t: SpecTecTyp },
    #[spectec_node(name = "typ")]
    Typ { x: String },
    #[spectec_node(name = "def")]
    Def {
        x: String,
        #[spectec_field(vec = true)]
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
    },
    #[spectec_node(name = "gram")]
    Gram { x: String, t: SpecTecTyp },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L196>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecInst {
    #[spectec_node(name = "inst")]
    Inst {
        #[spectec_field(vec = true)]
        bs: Vec<SpecTecBind>,
        #[spectec_field(vec = true)]
        as_: Vec<SpecTecArg>,
        dt: SpecTecDefTyp,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L201>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecRule {
    #[spectec_node(name = "rule")]
    Rule {
        x: String,
        #[spectec_field(vec = true)]
        bs: Vec<SpecTecBind>,
        op: MixOp,
        e: SpecTecExp,
        #[spectec_field(vec = true)]
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L206>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecClause {
    #[spectec_node(name = "clause")]
    Clause {
        #[spectec_field(vec = true)]
        bs: Vec<SpecTecBind>,
        #[spectec_field(vec = true)]
        as_: Vec<SpecTecArg>,
        e: SpecTecExp,
        #[spectec_field(vec = true)]
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L211>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecProd {
    #[spectec_node(name = "prod")]
    Prod {
        #[spectec_field(vec = true)]
        bs: Vec<SpecTecBind>,
        g: SpecTecSym,
        e: SpecTecExp,
        #[spectec_field(vec = true)]
        prs: Vec<SpecTecPrem>,
    },
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L216>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecDef {
    #[spectec_node(name = "typ")]
    Typ {
        x: String,
        #[spectec_field(vec = true)]
        ps: Vec<SpecTecParam>,
        #[spectec_field(vec = true)]
        insts: Vec<SpecTecInst>,
    },
    #[spectec_node(name = "rel")]
    Rel {
        x: String,
        op: MixOp,
        t: SpecTecTyp,
        #[spectec_field(vec = true)]
        rules: Vec<SpecTecRule>,
    },
    #[spectec_node(name = "def")]
    Dec {
        x: String,
        #[spectec_field(vec = true)]
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
        #[spectec_field(vec = true)]
        clauses: Vec<SpecTecClause>,
    },
    #[spectec_node(name = "gram")]
    Gram {
        x: String,
        #[spectec_field(vec = true)]
        ps: Vec<SpecTecParam>,
        t: SpecTecTyp,
        #[spectec_field(vec = true)]
        prods: Vec<SpecTecProd>,
    },
    #[spectec_node(name = "rec")]
    Rec {
        #[spectec_field(vec = true)]
        ds: Vec<SpecTecDef>,
    },
}

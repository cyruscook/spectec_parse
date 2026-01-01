use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#28>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecUnOp {
    #[sexpr_atom(name = "not")]
    Not,
    #[sexpr_atom(name = "plus")]
    Plus,
    #[sexpr_atom(name = "minus")]
    Minus,
    #[sexpr_atom(name = "plusminus")]
    PlusMinus,
    #[sexpr_atom(name = "minusplus")]
    MinusPlus,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#35>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecBinOp {
    #[sexpr_atom(name = "and")]
    And,
    #[sexpr_atom(name = "or")]
    Or,
    #[sexpr_atom(name = "impl")]
    Impl,
    #[sexpr_atom(name = "equiv")]
    Equiv,
    #[sexpr_atom(name = "add")]
    Add,
    #[sexpr_atom(name = "sub")]
    Sub,
    #[sexpr_atom(name = "mul")]
    Mul,
    #[sexpr_atom(name = "div")]
    Div,
    #[sexpr_atom(name = "mod")]
    Mod,
    #[sexpr_atom(name = "pow")]
    Pow,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#47>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecCmpOp {
    #[sexpr_atom(name = "eq")]
    Eq,
    #[sexpr_atom(name = "ne")]
    Ne,
    #[sexpr_atom(name = "lt")]
    Lt,
    #[sexpr_atom(name = "gt")]
    Gt,
    #[sexpr_atom(name = "le")]
    Le,
    #[sexpr_atom(name = "ge")]
    Ge,
}

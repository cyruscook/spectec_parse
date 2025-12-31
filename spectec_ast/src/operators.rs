use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#28>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecUnOp {
    #[spectec_atom(name = "not")]
    Not,
    #[spectec_atom(name = "plus")]
    Plus,
    #[spectec_atom(name = "minus")]
    Minus,
    #[spectec_atom(name = "plusminus")]
    PlusMinus,
    #[spectec_atom(name = "minusplus")]
    MinusPlus,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#35>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecBinOp {
    #[spectec_atom(name = "and")]
    And,
    #[spectec_atom(name = "or")]
    Or,
    #[spectec_atom(name = "impl")]
    Impl,
    #[spectec_atom(name = "equiv")]
    Equiv,
    #[spectec_atom(name = "add")]
    Add,
    #[spectec_atom(name = "sub")]
    Sub,
    #[spectec_atom(name = "mul")]
    Mul,
    #[spectec_atom(name = "div")]
    Div,
    #[spectec_atom(name = "mod")]
    Mod,
    #[spectec_atom(name = "pow")]
    Pow,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#47>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecCmpOp {
    #[spectec_atom(name = "eq")]
    Eq,
    #[spectec_atom(name = "ne")]
    Ne,
    #[spectec_atom(name = "lt")]
    Lt,
    #[spectec_atom(name = "gt")]
    Gt,
    #[spectec_atom(name = "le")]
    Le,
    #[spectec_atom(name = "ge")]
    Ge,
}

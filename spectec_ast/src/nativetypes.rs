use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/xl/bool.ml#L9>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecBoolTyp {
    #[sexpr_atom(name = "bool")]
    Bool,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/xl/num.ml#L27>
#[allow(unused)]
#[derive(SExprDecode, Clone, Debug, PartialEq)]
pub enum SpecTecNumTyp {
    #[sexpr_atom(name = "nat")]
    Nat,
    #[sexpr_atom(name = "int")]
    Int,
    #[sexpr_atom(name = "rat")]
    Rat,
    #[sexpr_atom(name = "real")]
    Real,
}

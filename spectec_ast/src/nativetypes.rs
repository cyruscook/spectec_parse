use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/xl/bool.ml#L9>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecBoolTyp {
    #[spectec_atom(name = "bool")]
    Bool,
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/xl/num.ml#L27>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecNumTyp {
    #[spectec_atom(name = "nat")]
    Nat,
    #[spectec_atom(name = "int")]
    Int,
    #[spectec_atom(name = "rat")]
    Rat,
    #[spectec_atom(name = "real")]
    Real,
}

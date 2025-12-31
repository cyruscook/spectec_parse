use crate::SpecTecExp;
use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#58>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecIter {
    #[spectec_atom(name = "opt")]
    Opt,
    #[spectec_atom(name = "list")]
    List,
    #[spectec_atom(name = "list1")]
    List1,
    #[spectec_node(name = "listn")]
    ListN {
        #[spectec_field(vec = true)]
        e: Vec<SpecTecExp>,
        #[spectec_field(vec = true)]
        xo: Vec<String>,
    },
}

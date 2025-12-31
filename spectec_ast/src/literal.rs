use spectec_derive::SpecTecItem;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L14>
#[derive(Debug, PartialEq)]
pub struct MixOp(Vec<String>);

impl spectec_decode::Decode for MixOp {
    fn can_decode(item: &sexpr::SExprItem) -> bool {
        matches!(item, sexpr::SExprItem::Text(_))
    }

    fn decode(item: sexpr::SExprItem) -> Result<Self, spectec_decode::DecodeError> {
        match item {
            sexpr::SExprItem::Text(t) => Ok(MixOp(t.split('%').map(str::to_owned).collect())),
            _ => Err(spectec_decode::DecodeError::UnexpectedItem(item)
                .with_context(format!("while decoding {}", std::any::type_name::<Self>()))),
        }
    }
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#19>
#[allow(unused)]
#[derive(SpecTecItem, Debug, PartialEq)]
pub enum SpecTecNum {
    #[spectec_node(name = "nat")]
    Nat(u64),
    #[spectec_node(name = "int")]
    Int(i64),
    #[spectec_node(name = "rat")]
    Rat(String),
    #[spectec_node(name = "real")]
    Real(String),
}

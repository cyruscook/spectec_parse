use crate::decode::Decode;
use crate::error::DecodeError;

impl Decode for String {
    fn can_decode(item: &sexpr::SExprItem) -> bool {
        matches!(item, sexpr::SExprItem::Text(_))
    }

    fn decode(item: sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            sexpr::SExprItem::Text(t) => Ok(t),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

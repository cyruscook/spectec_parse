use crate::decode::Decode;
use crate::error::DecodeError;

impl Decode for bool {
    fn can_decode(item: &sexpr::SExprItem) -> bool {
        match item {
            sexpr::SExprItem::Atom(t) => t.parse::<Self>().is_ok(),
            _ => false,
        }
    }

    fn decode(item: sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            sexpr::SExprItem::Atom(t) => t.parse().map_err(DecodeError::from),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

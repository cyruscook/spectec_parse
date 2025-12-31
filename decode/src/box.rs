use crate::decode::Decode;
use crate::error::DecodeError;

impl<T: Decode> Decode for Box<T> {
    fn can_decode(item: &sexpr::SExprItem) -> bool {
        T::can_decode(item)
    }

    fn decode(item: sexpr::SExprItem) -> Result<Self, DecodeError> {
        T::decode(item).map(Self::new).map_err(|e| {
            e.with_context(format!("while decoding {}", std::any::type_name::<Self>()))
        })
    }
}

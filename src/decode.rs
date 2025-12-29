use std::iter::Peekable;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unrecognised symbol '{0}'")]
    UnrecognisedSymbol(String),
    #[error("Unexpected item '{0:?}'")]
    UnexpectedItem(crate::sexpr::SExprItem),
    #[error("Error reading SExpr: {0}")]
    SExpr(#[from] crate::sexpr::SExprError),
}

pub trait Decode: Sized {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool;
    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError>;
}

impl Decode for String {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        matches!(item, crate::sexpr::SExprItem::Text(_))
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Text(t) => Ok(t),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
    }
}

pub fn decode_iter<V: Into<Vec<T>>, T: Decode, I: Iterator<Item = crate::sexpr::SExprItem>>(
    items: &mut Peekable<I>,
) -> Result<Vec<T>, DecodeError> {
    let mut parsed = Vec::new();
    while let Some(item) = items.peek() {
        if T::can_decode(item) {
            // We know that there is another item as we've checked the result of peek
            #[allow(clippy::unwrap_used)]
            let item = items.next().unwrap();
            parsed.push(T::decode(item)?);
        } else {
            // T cannot accept the item. Stop processing
            break;
        }
    }
    Ok(parsed)
}

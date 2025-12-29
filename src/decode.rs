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

pub(crate) trait Decode: Sized {
    /// Checks if the s-expression item can be decoded into Self.
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool;

    /// Decodes s-expression item into `Self`.
    ///
    /// # Errors
    ///
    /// Will return an error if the s-expression cannot be represented by `Self`. To avoid this case,
    /// use `can_decode` to check if the item can be decoded first.
    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError>;

    /// Decodes multiple items from an iterator into a `Vec<Self>`.
    /// Stops processing when an item is encountered that `T` cannot decode.
    fn decode_iter<I: Iterator<Item = crate::sexpr::SExprItem>>(
        items: &mut Peekable<I>,
    ) -> Result<Vec<Self>, DecodeError> {
        let mut parsed = Vec::new();
        while let Some(item) = items.peek() {
            if Self::can_decode(item) {
                // We know that there is another item as we've checked the result of peek
                #[allow(clippy::unwrap_used)]
                let item = items.next().unwrap();
                parsed.push(Self::decode(item)?);
            } else {
                // T cannot accept the item. Stop processing
                break;
            }
        }
        Ok(parsed)
    }
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

/// Decodes multiple items from an iterator into a Vec<T>.
/// Stops processing when an item is encountered that T cannot decode.
/// This is a convenience wrapper around T::decode_iter, which can be used in macros to avoid the
/// need to specify T explicitly, instead the compiler can infer T from V (Vec<T>).
#[allow(unused, clippy::extra_unused_type_parameters)]
pub(crate) fn decode_iter<
    V: Into<Vec<T>>,
    T: Decode,
    I: Iterator<Item = crate::sexpr::SExprItem>,
>(
    items: &mut Peekable<I>,
) -> Result<Vec<T>, DecodeError> {
    T::decode_iter(items)
}

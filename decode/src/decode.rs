use crate::error::DecodeError;
use std::iter::Peekable;

pub trait Decode: Sized {
    /// Checks if the s-expression item can be decoded into Self.
    fn can_decode(item: &sexpr::SExprItem) -> bool;

    /// Decodes s-expression item into `Self`.
    ///
    /// # Errors
    ///
    /// Will return an error if the s-expression cannot be represented by `Self`. To avoid this case,
    /// use `can_decode` to check if the item can be decoded first.
    fn decode(item: sexpr::SExprItem) -> Result<Self, DecodeError>;

    /// Takes from an iterator all the items that can be decoded into a Vec<T>.
    /// Always returns true, as we can decode the items we take from the iterator (even if none)
    fn can_decode_iter<'a, I: Iterator<Item = &'a sexpr::SExprItem>>(
        items: &mut Peekable<I>,
    ) -> bool {
        while let Some(item) = items.peek() {
            if Self::can_decode(item) {
                // We know that there is another item as we've checked the result of peek
                #[allow(clippy::unwrap_used)]
                items.next().unwrap();
            } else {
                break;
            }
        }
        true
    }

    /// Decodes as many items from an iterator as we can.
    /// Stops processing when an item is encountered that `T` cannot decode.
    fn decode_iter<I: Iterator<Item = sexpr::SExprItem>>(
        items: &mut Peekable<I>,
    ) -> Result<Vec<Self>, DecodeError> {
        let mut parsed = Vec::new();
        while let Some(item) = items.peek() {
            if Self::can_decode(item) {
                // We know that there is another item as we've checked the result of peek
                #[allow(clippy::unwrap_used)]
                let item = items.next().unwrap();
                parsed.push(Self::decode(item).map_err(|e| {
                    e.with_context(format!("while decoding {}", std::any::type_name::<Self>()))
                })?);
            } else {
                // T cannot accept the item. Stop processing
                break;
            }
        }
        Ok(parsed)
    }
}

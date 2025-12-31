use crate::decode::Decode;
use crate::error::DecodeError;
use std::iter::Peekable;

/// Takes from an iterator one item, only if it could be decoded as T.
/// Always returns true.
#[allow(
    unused,
    clippy::extra_unused_type_parameters,
    clippy::missing_panics_doc
)]
pub fn can_decode_option<
    'a,
    V: From<Option<T>>,
    T: Decode,
    I: Iterator<Item = &'a sexpr::SExprItem>,
>(
    items: &mut Peekable<I>,
) -> bool {
    if let Some(item) = items.peek()
        && T::can_decode(item)
    {
        // We know that there is another item as we've checked the result of peek
        #[allow(clippy::unwrap_used)]
        items.next().unwrap();
    }
    true
}

/// Takes from an iterator one item, only if it could be decoded as `T`, in which case the result
/// of decoding that item as `T` is returned, otherwise `Ok(None)` is returned.
#[allow(
    unused,
    clippy::extra_unused_type_parameters,
    clippy::missing_panics_doc
)]
pub fn decode_option<V: From<Option<T>>, T: Decode, I: Iterator<Item = sexpr::SExprItem>>(
    items: &mut Peekable<I>,
) -> Result<Option<T>, DecodeError> {
    if let Some(item) = items.peek()
        && T::can_decode(item)
    {
        // We know that there is another item as we've checked the result of peek
        #[allow(clippy::unwrap_used)]
        return T::decode(items.next().unwrap()).map(Some);
    }
    Ok(None)
}

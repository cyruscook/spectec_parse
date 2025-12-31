use crate::decode::Decode;
use crate::error::DecodeError;
use std::iter::Peekable;

/// Takes from an iterator all the items that can be decoded into a Vec<T>.
/// Always returns true, as we can decode the items we take from the iterator (even if none)
/// This is a convenience wrapper around T::can_decode_iter, which can be used in macros to avoid
/// the need to specify T explicitly, instead the compiler can infer T from V (Vec<T>).
#[allow(unused, clippy::extra_unused_type_parameters)]
pub fn can_decode_iter<'a, V: Into<Vec<T>>, T: Decode, I: Iterator<Item = &'a sexpr::SExprItem>>(
    items: &mut Peekable<I>,
) -> bool {
    T::can_decode_iter(items)
}

/// Decodes as many items from an iterator as we can.
/// Stops processing when an item is encountered that `T` cannot decode.
/// This is a convenience wrapper around T::decode_iter, which can be used in macros to avoid the
/// need to specify T explicitly, instead the compiler can infer T from V (Vec<T>).
#[allow(unused, clippy::extra_unused_type_parameters)]
pub fn decode_iter<V: Into<Vec<T>>, T: Decode, I: Iterator<Item = sexpr::SExprItem>>(
    items: &mut Peekable<I>,
) -> Result<Vec<T>, DecodeError> {
    T::decode_iter(items)
}

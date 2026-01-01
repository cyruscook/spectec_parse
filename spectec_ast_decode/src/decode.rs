pub trait Decode: Sized {
    /// Consumes zero or more s-expression items from the iterator to construct `Self`.
    ///
    /// # Errors
    ///
    /// Will return an error if the s-expression cannot be represented by `Self`. To avoid this case,
    /// use `can_decode` to check if the item can be decoded first.
    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self>;
}

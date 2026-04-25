pub trait Decode: Sized {
    /// Consumes zero or more S-expression items from the iterator to construct `Self`.
    ///
    /// # Errors
    ///
    /// Will return an error if the S-expression cannot be represented by `Self`. To avoid this case,
    /// use `can_decode` to check if the item can be decoded first.
    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self>;

    /// Attempts to decode `T` from a single item and reports success only if `T`
    /// consumed that item completely.
    ///
    /// This is stricter than treating any `Ok(T)` as a match: nested greedy
    /// decoders such as `Vec<_>` or `Option<_>` can return success without
    /// consuming input, which must not be interpreted as presence by outer greedy
    /// decoders.
    #[must_use]
    fn probe_one(item: &sexpr_parse::SExprItem) -> Option<Self> {
        let mut probe = std::iter::once(item).peekable();
        let out = Self::decode(&mut probe).ok()?;

        if probe.peek().is_none() {
            Some(out)
        } else {
            None
        }
    }
}

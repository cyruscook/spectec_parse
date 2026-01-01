impl<T: crate::Decode> crate::Decode for Option<T> {
    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        if let Some(item) = items.peek()
            && let Ok(out) = T::decode(&mut std::iter::once(*item).peekable())
        {
            // We know that an item is available due to the success of the peek call
            #[allow(clippy::unwrap_used)]
            items.next().unwrap();
            return Ok(Some(out));
        }
        Ok(None)
    }
}

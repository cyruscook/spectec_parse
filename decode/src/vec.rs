impl<T: crate::Decode> crate::Decode for Vec<T> {
    fn decode<'a, I: Iterator<Item = &'a sexpr::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        let mut parsed = Vec::new();
        while let Some(item) = items.peek() {
            if let Ok(out) = T::decode(&mut std::iter::once(*item).peekable()) {
                // We know that an item is available due to the success of the peek call
                #[allow(clippy::unwrap_used)]
                items.next().unwrap();
                parsed.push(out);
            } else {
                break;
            }
        }
        Ok(parsed)
    }
}

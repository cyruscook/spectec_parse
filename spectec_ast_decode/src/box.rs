impl<T: crate::Decode> crate::Decode for Box<T> {
    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        T::decode(items)
            .map(Self::new)
            .map_err(crate::Error::wrapped::<Self>)
    }
}

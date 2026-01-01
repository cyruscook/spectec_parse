impl crate::Decode for String {
    fn decode<'a, I: Iterator<Item = &'a sexpr::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        match items.next() {
            Some(sexpr::SExprItem::Text(t)) => Ok(t.clone()),
            Some(item) => Err(crate::Error::cannot_decode_sexpr::<Self>(item)),
            None => Err(crate::Error::required_missing_sexpr::<Self>()),
        }
    }
}

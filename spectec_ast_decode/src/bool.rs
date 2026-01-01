impl crate::Decode for bool {
    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        match items.next() {
            Some(sexpr_parse::SExprItem::Atom(t)) => {
                t.parse().map_err(crate::Error::parse_bool_err::<Self>)
            }
            Some(item) => Err(crate::Error::cannot_decode_sexpr::<Self>(item)),
            None => Err(crate::Error::required_missing_sexpr::<Self>()),
        }
    }
}

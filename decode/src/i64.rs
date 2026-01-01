fn parse_i64_str(s: &str) -> Result<i64, std::num::ParseIntError> {
    if let Some(stripped) = s.strip_prefix("0x") {
        i64::from_str_radix(stripped, 16)
    } else {
        s.parse::<i64>()
    }
}

impl crate::Decode for i64 {
    fn decode<'a, I: Iterator<Item = &'a sexpr::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        match items.next() {
            Some(sexpr::SExprItem::Atom(t)) => {
                parse_i64_str(t).map_err(crate::Error::parse_int_err::<Self>)
            }
            Some(item) => Err(crate::Error::cannot_decode_sexpr::<Self>(item)),
            None => Err(crate::Error::required_missing_sexpr::<Self>()),
        }
    }
}

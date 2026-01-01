fn parse_u64_str(s: &str) -> Result<u64, std::num::ParseIntError> {
    if let Some(stripped) = s.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else {
        s.parse::<u64>()
    }
}

impl crate::Decode for u64 {
    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        match items.next() {
            Some(sexpr_parse::SExprItem::Atom(t)) => {
                parse_u64_str(t).map_err(crate::Error::parse_int_err::<Self>)
            }
            Some(item) => Err(crate::Error::cannot_decode_sexpr::<Self>(item)),
            None => Err(crate::Error::required_missing_sexpr::<Self>()),
        }
    }
}

impl<T: crate::Decode> crate::Decode for Option<T> {
    fn decode<'a, I: Iterator<Item = &'a sexpr_parse::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> crate::Result<Self> {
        if let Some(item) = items.peek()
            && let Some(out) = crate::probe_one::<T>(item)
        {
            // We know that an item is available due to the success of the peek call
            #[allow(clippy::unwrap_used)]
            items.next().unwrap();
            return Ok(Some(out));
        }
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use crate::Decode;
    use sexpr_parse::SExprItem;

    #[test]
    fn nested_vec_probe_does_not_match_without_consuming() {
        let items = [SExprItem::Atom("x".to_owned())];
        let mut iter = items.iter().peekable();

        let out = Option::<Vec<u64>>::decode(&mut iter).unwrap();

        assert_eq!(out, None);
        assert_eq!(iter.next(), Some(&SExprItem::Atom("x".to_owned())));
    }

    #[test]
    fn nested_option_probe_does_not_match_without_consuming() {
        let items = [SExprItem::Atom("x".to_owned())];
        let mut iter = items.iter().peekable();

        let out = Option::<Option<u64>>::decode(&mut iter).unwrap();

        assert_eq!(out, None);
        assert_eq!(iter.next(), Some(&SExprItem::Atom("x".to_owned())));
    }
}

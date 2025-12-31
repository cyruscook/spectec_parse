use crate::decode::Decode;
use crate::error::DecodeError;
use std::num::ParseIntError;

fn parse_i64_str(s: &str) -> Result<i64, ParseIntError> {
    if let Some(stripped) = s.strip_prefix("0x") {
        i64::from_str_radix(stripped, 16)
    } else {
        s.parse::<i64>()
    }
}

impl Decode for i64 {
    fn can_decode(item: &sexpr::SExprItem) -> bool {
        match item {
            sexpr::SExprItem::Atom(t) => parse_i64_str(t).is_ok(),
            _ => false,
        }
    }

    fn decode(item: sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            sexpr::SExprItem::Atom(t) => parse_i64_str(&t).map_err(DecodeError::from),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

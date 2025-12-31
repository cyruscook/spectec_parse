use crate::decode::Decode;
use crate::error::DecodeError;
use std::num::ParseIntError;

fn parse_u64_str(s: &str) -> Result<u64, ParseIntError> {
    if let Some(stripped) = s.strip_prefix("0x") {
        u64::from_str_radix(stripped, 16)
    } else {
        s.parse::<u64>()
    }
}

impl Decode for u64 {
    fn can_decode(item: &sexpr::SExprItem) -> bool {
        match item {
            sexpr::SExprItem::Atom(t) => parse_u64_str(t).is_ok(),
            _ => false,
        }
    }

    fn decode(item: sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            sexpr::SExprItem::Atom(t) => parse_u64_str(&t).map_err(DecodeError::from),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

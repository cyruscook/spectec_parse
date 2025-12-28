use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unrecognised symbol '{0}'")]
    UnrecognisedSymbol(String),
    #[error("Unexpected item '{0:?}'")]
    UnexpectedItem(crate::sexpr::SExprItem),
    #[error("Error reading SExpr: {0}")]
    SExpr(#[from] crate::sexpr::SExprError),
}

pub trait Decode: Sized {
    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError>;
}

impl Decode for String {
    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Text(t) => Ok(t.clone()),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
    }
}

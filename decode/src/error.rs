use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unrecognised symbol '{0}'")]
    UnrecognisedSymbol(String),
    #[error("Missing an expected item")]
    MissingItem,
    #[error("Unexpected item '{0:?}'")]
    UnexpectedItem(sexpr::SExprItem),
    #[error("Error reading SExpr: {0}")]
    SExpr(#[from] sexpr::SExprError),
    #[error("Error parsing a bool: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("Error parsing an int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{context}: {source}")]
    WithContext {
        #[source]
        source: Box<DecodeError>,
        context: String,
    },
}

impl DecodeError {
    #[must_use]
    pub fn with_context<R: AsRef<str>>(self, context: R) -> Self {
        Self::WithContext {
            source: Box::new(self),
            context: context.as_ref().to_string(),
        }
    }
}

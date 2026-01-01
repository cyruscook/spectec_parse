use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Error parsing S-expression: {0}")]
    SExpr(#[from] sexpr::SExprError),
    #[error("Error decoding SpecTec AST: {0}")]
    Decode(#[from] decode::Error),
}

pub type Result<T> = core::result::Result<T, Error>;

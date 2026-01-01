#![deny(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

mod error;
mod parse;
mod reader;
mod sexpr;

pub use error::SExprError;
pub use parse::parse_sexpr_stream;
pub use sexpr::SExprItem;

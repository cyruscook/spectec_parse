#![deny(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

mod bool;
mod r#box;
mod decode;
mod error;
mod i64;
mod iter;
mod option;
mod string;
mod u64;

#[allow(unused)]
pub use crate::{
    bool::*, r#box::*, decode::*, error::*, i64::*, iter::*, option::*, string::*, u64::*,
};

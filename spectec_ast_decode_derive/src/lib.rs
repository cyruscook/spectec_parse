#![deny(
    clippy::arithmetic_side_effects,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

extern crate proc_macro;

mod item;
mod utils;

use synstructure::decl_derive;

decl_derive!([SExprDecode, attributes(sexpr_node, sexpr_atom)] => item::sexpr_decode_derive);

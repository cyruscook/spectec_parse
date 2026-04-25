#![deny(
    clippy::arithmetic_side_effects,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::pedantic
)]
#![cfg_attr(test, allow(clippy::unwrap_used, clippy::expect_used, clippy::panic))]
#![allow(clippy::doc_markdown, clippy::missing_errors_doc)]

pub use spectec_ast;

const NANO_WASM_AST_STR: &str = include_str!("./NanoWasm.spectec-ast");

/// Returns the NanoWASM SpecTec AST
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn get_nano_wasm_spectec_ast() -> Vec<spectec_ast::SpecTecDef> {
    // SAFETY: The included SpecTec AST is known to be valid.
    #[allow(clippy::panic)]
    match spectec_ast::parse_spectec_stream(NANO_WASM_AST_STR) {
        Ok(ast) => ast,
        Err(err) => panic!("Failed to parse known valid NanoWASM SpecTec AST: {err}"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_nano_wasm_spectec_ast() {
        let ast = super::get_nano_wasm_spectec_ast();
        assert!(!ast.is_empty());
    }
}

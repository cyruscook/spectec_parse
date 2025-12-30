const WASM_AST_STR: &str = include_str!("./wasm-3.0.spectec-ast");

/// Returns the WASM SpecTec AST
#[must_use]
#[allow(clippy::missing_panics_doc)]
pub fn get_wasm_spectec_ast() -> Vec<crate::spectec::SpecTecDef> {
    // SAFETY: The included WASM SpecTec AST is known to be valid.
    #[allow(clippy::panic)]
    match crate::parse_spectec_stream(WASM_AST_STR) {
        Ok(ast) => ast,
        Err(err) => panic!("Failed to parse known valid WASM SpecTec AST: {err}"),
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_wasm_spectec_ast() {
        let ast = super::get_wasm_spectec_ast();
        assert!(!ast.is_empty());
    }
}

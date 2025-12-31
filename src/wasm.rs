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
    use crate::{decode::Decode, spectec::SpecTecDef};

    #[test]
    fn test_get_wasm_spectec_ast() {
        //let ast = super::get_wasm_spectec_ast();
        //assert!(!ast.is_empty());
    }

    #[test]
    fn test_spectec_wasm1() {
        let input = r#"
            (def
              "opt_"
              (typ "X")
              (exp "_" (iter (var "X") list))
              (iter (var "X") opt)
              (clause (typ "X") (typ (var "X")) (exp (list)) (opt))
              (clause
                (opt (var "w"))
              )
            )
                "#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| match SpecTecDef::decode(sexpr) {
                Ok(p) => p,
                Err(e) => panic!("{}", e),
            })
            .collect::<Vec<_>>();
        dbg!(&parsed);
        //assert_eq!(parsed, vec![]);
    }
}

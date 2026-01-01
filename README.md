# spectec_parse

A collection of tools for handling the WebAssembly specification as a SpecTec AST.

The WebAssembly specification is now written in SpecTec, which handles the generation of the WebAssembly specification website and documentation.

These crates can be used to consume the SpecTec AST of the WebAssembly specification in Rust tools, for example to generate documentation or track changes.

Crates:
* [wasm_spec_ast](./wasm_spec_ast) - The WebAssembly spec as a SpecTec AST
* [spectec_ast](./spectec_ast) - Parser for SpecTec ASTs in S-expression format
* [sexpr_parse](./sexpr_parse) - Parser for S-expressions
* [spectec_ast_decode](./spectec_ast_decode) - Trait for decoding SpecTec AST S-expressions
* [spectec_ast_decode_derive](./spectec_ast_decode_derive) - Proc macro for generating decoding implementations for SpecTec AST S-expressions

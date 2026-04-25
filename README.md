# spectec_parse

A collection of tools for handling the WebAssembly specification as a SpecTec AST.

The WebAssembly specification is now written in SpecTec, which handles the generation of the WebAssembly specification website and documentation.

These crates can be used to consume the SpecTec AST of the WebAssembly specification in Rust tools, for example to generate documentation or track changes.

Crates:
* [wasm_spec_ast](./wasm_spec_ast) - The WebAssembly SpecTec specification ([crates.io](https://crates.io/crates/wasm_spec_ast), [docs.rs](https://docs.rs/wasm_spec_ast))
* [nano_wasm_spec_ast](./nano_wasm_spec_ast) - The demo language "NanoWASM" SpecTec specification
* [spectec_ast](./spectec_ast) - Parser for SpecTec ASTs in S-expression format ([crates.io](https://crates.io/crates/spectec_ast), [docs.rs](https://docs.rs/spectec_ast))
* [sexpr_parse](./sexpr_parse) - Parser for S-expressions ([crates.io](https://crates.io/crates/sexpr_parse), [docs.rs](https://docs.rs/sexpr_parse))
* [spectec_ast_decode](./spectec_ast_decode) - Trait for decoding SpecTec AST S-expressions ([crates.io](https://crates.io/crates/spectec_ast_decode), [docs.rs](https://docs.rs/spectec_ast_decode))
* [spectec_ast_decode_derive](./spectec_ast_decode_derive) - Proc macro for generating decoding implementations for SpecTec AST S-expressions ([crates.io](https://crates.io/crates/spectec_ast_decode_derive), [docs.rs](https://docs.rs/spectec_ast_decode_derive))

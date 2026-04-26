# Contributing

To regenerate `wasm-3.0.spectec-ast`:

1. Follow the instructions to build SpecTec: https://github.com/WebAssembly/spec/blob/d7b678327cd370cdbc5acfa94bd108772e2bef68/spectec/README.md
2. Run this command to compile the WASM 3.0 specification: https://github.com/WebAssembly/spec/tree/d7b678327cd370cdbc5acfa94bd108772e2bef68/specification/

```sh
spectec ./wasm-3.0/** --ast > wasm-3.0.spectec-ast
```

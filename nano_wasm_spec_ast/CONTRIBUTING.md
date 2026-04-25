# Contributing

To regenerate `NanoWasm.spectec-ast`:

1. Follow the instructions to build SpecTec: https://github.com/WebAssembly/spec/blob/d7b678327cd370cdbc5acfa94bd108772e2bef68/spectec/README.md
2. Run this command to compile the NanoWASM specification: https://github.com/WebAssembly/spec/blob/d7b678327cd370cdbc5acfa94bd108772e2bef68/spectec/doc/example/NanoWasm.spectec

```sh
spectec ./NanoWasm.spectec --ast -o NanoWasm.spectec-ast
```

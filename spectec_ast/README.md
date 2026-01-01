# spectec_ast

Parser for SpecTec ASTs in S-expression format.

Reads S-expressions from the sexpr_parse crate into the AST format.

## Usage:

```rust
let input = r#"(typ "m" (inst (alias nat)))"#;
let parsed = parse_spectec_stream(input).unwrap();
assert_eq!(
    parsed,
    vec![SpecTecDef::Typ {
        x: "m".to_string(),
        insts: vec![SpecTecInst::Inst {
            bs: vec![],
            as_: vec![],
            dt: SpecTecDefTyp::Alias {
                typ: SpecTecTyp::Num(SpecTecNumTyp::Nat),
            },
        }],
        ps: vec![],
    },]
);
```

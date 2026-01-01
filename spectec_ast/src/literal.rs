use decode_derive::SExprDecode;

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L14>
#[derive(Debug, PartialEq)]
pub struct MixOp(Vec<String>);

impl decode::Decode for MixOp {
    fn decode<'a, I: Iterator<Item = &'a sexpr::SExprItem>>(
        items: &mut std::iter::Peekable<I>,
    ) -> decode::Result<Self> {
        match items.next() {
            Some(sexpr::SExprItem::Text(t)) => Ok(MixOp(t.split('%').map(str::to_owned).collect())),
            Some(item) => Err(decode::Error::cannot_decode_sexpr::<Self>(item)),
            None => Err(decode::Error::required_missing_sexpr::<Self>()),
        }
    }
}

/// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#19>
#[allow(unused)]
#[derive(SExprDecode, Debug, PartialEq)]
pub enum SpecTecNum {
    #[sexpr_node(name = "nat")]
    Nat(u64),
    #[sexpr_node(name = "int")]
    Int(i64),
    #[sexpr_node(name = "rat")]
    Rat(String),
    #[sexpr_node(name = "real")]
    Real(String),
}

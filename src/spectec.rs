mod nativetypes {
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/xl/bool.ml#L9>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecBoolTyp {
        #[spectec_atom(name = "bool")]
        Bool,
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/xl/num.ml#L27>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecNumTyp {
        #[spectec_atom(name = "nat")]
        Nat,
        #[spectec_atom(name = "int")]
        Int,
        #[spectec_atom(name = "rat")]
        Rat,
        #[spectec_atom(name = "real")]
        Real,
    }
}
pub use nativetypes::*;

mod literal {
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L14>
    #[derive(Debug, PartialEq)]
    pub struct MixOp(Vec<String>);

    impl crate::decode::Decode for MixOp {
        fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
            matches!(item, crate::sexpr::SExprItem::Text(_))
        }

        fn decode(item: crate::sexpr::SExprItem) -> Result<Self, crate::decode::DecodeError> {
            match item {
                crate::sexpr::SExprItem::Text(t) => {
                    Ok(MixOp(t.split('%').map(str::to_owned).collect()))
                }
                _ => Err(crate::decode::DecodeError::UnexpectedItem(item)
                    .with_context(format!("while decoding {}", std::any::type_name::<Self>()))),
            }
        }
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#19>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecNum {
        #[spectec_node(name = "nat")]
        Nat(u64),
        #[spectec_node(name = "int")]
        Int(i64),
        #[spectec_node(name = "rat")]
        Rat(String),
        #[spectec_node(name = "real")]
        Real(String),
    }
}
pub use literal::*;

mod operators {
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#28>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecUnOp {
        #[spectec_atom(name = "not")]
        Not,
        #[spectec_atom(name = "plus")]
        Plus,
        #[spectec_atom(name = "minus")]
        Minus,
        #[spectec_atom(name = "plusminus")]
        PlusMinus,
        #[spectec_atom(name = "minusplus")]
        MinusPlus,
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#35>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecBinOp {
        #[spectec_atom(name = "and")]
        And,
        #[spectec_atom(name = "or")]
        Or,
        #[spectec_atom(name = "impl")]
        Impl,
        #[spectec_atom(name = "equiv")]
        Equiv,
        #[spectec_atom(name = "add")]
        Add,
        #[spectec_atom(name = "sub")]
        Sub,
        #[spectec_atom(name = "mul")]
        Mul,
        #[spectec_atom(name = "div")]
        Div,
        #[spectec_atom(name = "mod")]
        Mod,
        #[spectec_atom(name = "pow")]
        Pow,
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#47>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecCmpOp {
        #[spectec_atom(name = "eq")]
        Eq,
        #[spectec_atom(name = "ne")]
        Ne,
        #[spectec_atom(name = "lt")]
        Lt,
        #[spectec_atom(name = "gt")]
        Gt,
        #[spectec_atom(name = "le")]
        Le,
        #[spectec_atom(name = "ge")]
        Ge,
    }
}
pub use operators::*;

mod iterations {
    use crate::spectec::SpecTecExp;
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#58>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecIter {
        #[spectec_atom(name = "opt")]
        Opt,
        #[spectec_atom(name = "list")]
        List,
        #[spectec_atom(name = "list1")]
        List1,
        #[spectec_node(name = "listn")]
        ListN {
            #[spectec_field(vec = true)]
            e: Vec<SpecTecExp>,
            #[spectec_field(vec = true)]
            xo: Vec<String>,
        },
    }
}
pub use iterations::*;

mod types {
    use crate::spectec::{
        SpecTecArg, SpecTecBind, SpecTecBoolTyp, SpecTecExp, SpecTecIter, SpecTecNumTyp,
        SpecTecPrem,
    };
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L70>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecOpTyp {
        #[spectec_atom()]
        Num(SpecTecNumTyp),
        #[spectec_atom()]
        Bool(SpecTecBoolTyp),
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#74>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecTyp {
        #[spectec_node(name = "var")]
        Var {
            x: String,
            #[spectec_field(vec = true)]
            as1: Vec<SpecTecArg>,
        },
        #[spectec_atom(name = "bool")]
        Bool,
        #[spectec_atom()]
        Num(SpecTecNumTyp),
        #[spectec_atom(name = "text")]
        Text,
        #[spectec_node(name = "tup")]
        Tup {
            #[spectec_field(vec = true)]
            ets: Vec<SpecTecTypBind>,
        },
        #[spectec_node(name = "iter")]
        Iter {
            t1: Box<SpecTecTyp>,
            #[spectec_field(vec = true)]
            it: Vec<SpecTecIter>,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#83>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecDefTyp {
        #[spectec_node(name = "alias")]
        Alias { typ: SpecTecTyp },
        #[spectec_node(name = "struct")]
        Struct {
            #[spectec_field(vec = true)]
            tfs: Vec<SpecTecTypField>,
        },
        #[spectec_node(name = "variant")]
        Variant {
            #[spectec_field(vec = true)]
            tcs: Vec<SpecTecTypCase>,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#89>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecTypBind {
        #[spectec_node(name = "bind")]
        Bind { exp: SpecTecExp, typ: SpecTecTyp },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#92>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecTypField {
        #[spectec_node(name = "field")]
        Field {
            at: crate::spectec::literal::MixOp,
            #[spectec_field(vec = true)]
            bs: Vec<SpecTecBind>,
            t: SpecTecTyp,
            #[spectec_field(vec = true)]
            prs: Vec<SpecTecPrem>,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#95>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecTypCase {
        #[spectec_node(name = "case")]
        Field {
            op: crate::spectec::literal::MixOp,
            #[spectec_field(vec = true)]
            bs: Vec<SpecTecBind>,
            t: SpecTecTyp,
            #[spectec_field(vec = true)]
            prs: Vec<SpecTecPrem>,
        },
    }
}
use spectec_derive::SpecTecItem;
pub use types::*;

mod expressions {
    use crate::spectec::{
        MixOp, SpecTecArg, SpecTecBinOp, SpecTecCmpOp, SpecTecIter, SpecTecNum, SpecTecOpTyp,
        SpecTecTyp, SpecTecUnOp,
    };
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#101>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecExp {
        #[spectec_node(name = "var")]
        Var { id: String },
        #[spectec_node(name = "bool")]
        Bool { b: bool },
        #[spectec_node(name = "num")]
        Num { n: SpecTecNum },
        #[spectec_node(name = "text")]
        Text { t: String },
        #[spectec_node(name = "un")]
        Un {
            op: SpecTecUnOp,
            t: SpecTecOpTyp,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "bin")]
        Bin {
            op: SpecTecBinOp,
            t: SpecTecOpTyp,
            e1: Box<SpecTecExp>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "cmp")]
        Cmp {
            op: SpecTecCmpOp,
            t: SpecTecOpTyp,
            e1: Box<SpecTecExp>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "idx")]
        Idx {
            e1: Box<SpecTecExp>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "slice")]
        Slice {
            e1: Box<SpecTecExp>,
            e2: Box<SpecTecExp>,
            e3: Box<SpecTecExp>,
        },
        #[spectec_node(name = "upd")]
        Upd {
            e1: Box<SpecTecExp>,
            path: Box<SpecTecPath>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "ext")]
        Ext {
            e1: Box<SpecTecExp>,
            path: Box<SpecTecPath>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "struct")]
        Str {
            #[spectec_field(vec = true)]
            efs: Vec<SpecTecExpField>,
        },
        #[spectec_node(name = "dot")]
        Dot { e1: Box<SpecTecExp>, at: MixOp },
        #[spectec_node(name = "comp")]
        Comp {
            e1: Box<SpecTecExp>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "mem")]
        Mem {
            e1: Box<SpecTecExp>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "len")]
        Len { e1: Box<SpecTecExp> },
        #[spectec_node(name = "tup")]
        Tup {
            #[spectec_field(vec = true)]
            es: Vec<SpecTecExp>,
        },
        #[spectec_node(name = "call")]
        Call {
            x: String,
            #[spectec_field(vec = true)]
            as1: Vec<SpecTecArg>,
        },
        #[spectec_node(name = "iter")]
        Iter {
            e1: Box<SpecTecExp>,
            it: SpecTecIter,
            #[spectec_field(vec = true)]
            xes: Vec<SpecTecIterExp>,
        },
        #[spectec_node(name = "proj")]
        Proj { e1: Box<SpecTecExp>, i: i64 },
        #[spectec_node(name = "case")]
        Case { op: MixOp, e1: Box<SpecTecExp> },
        #[spectec_node(name = "uncase")]
        Uncase { e1: Box<SpecTecExp>, op: MixOp },
        #[spectec_node(name = "opt")]
        Opt {
            #[spectec_field(vec = true)]
            eo: Vec<Option<SpecTecExp>>,
        },
        #[spectec_node(name = "unopt")]
        Unopt { e1: Box<SpecTecExp> },
        #[spectec_node(name = "list")]
        List {
            #[spectec_field(vec = true)]
            es: Vec<SpecTecExp>,
        },
        #[spectec_node(name = "lift")]
        Lift { e1: Box<SpecTecExp> },
        #[spectec_node(name = "cat")]
        Cat {
            e1: Box<SpecTecExp>,
            e2: Box<SpecTecExp>,
        },
        #[spectec_node(name = "cvt")]
        Cvt {
            nt1: String,
            nt2: String,
            e1: Box<SpecTecExp>,
        },
        #[spectec_node(name = "sub")]
        Sub {
            t1: SpecTecTyp,
            t2: SpecTecTyp,
            e1: Box<SpecTecExp>,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#133>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecExpField {
        #[spectec_node(name = "field")]
        Field { at: MixOp, e: SpecTecExp },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#136>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecPath {
        #[spectec_atom(name = "root")]
        Root,
        #[spectec_node(name = "idx")]
        Idx { p1: Box<SpecTecPath>, e: SpecTecExp },
        #[spectec_node(name = "slice")]
        Slice {
            p1: Box<SpecTecPath>,
            e1: SpecTecExp,
            e2: SpecTecExp,
        },
        #[spectec_node(name = "dot")]
        Dot { p1: Box<SpecTecPath>, at: MixOp },
    }

    // Usage of this type must be preceded with a `crate::spectec::iterations::SpecTecIter` value, not included here
    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#143>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecIterExp {
        #[spectec_node(name = "dom")]
        Dom { x: String, e: SpecTecExp },
    }
}
pub use expressions::*;

mod grammars {
    use crate::spectec::{SpecTecArg, SpecTecExp, SpecTecIter, SpecTecIterExp};
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L149>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecSym {
        #[spectec_node(name = "var")]
        Var {
            x: String,
            #[spectec_field(vec = true)]
            as1: Vec<SpecTecArg>,
        },
        #[spectec_node(name = "num")]
        Num { n: i64 },
        #[spectec_node(name = "text")]
        Text { t: String },
        #[spectec_atom(name = "eps")]
        Eps,
        #[spectec_node(name = "seq")]
        Seq { gs: Box<SpecTecSym> },
        #[spectec_node(name = "alt")]
        Alt { gs: Box<SpecTecSym> },
        #[spectec_node(name = "range")]
        Range {
            g1: Box<SpecTecSym>,
            g2: Box<SpecTecSym>,
        },
        #[spectec_node(name = "iter")]
        Iter {
            g1: Box<SpecTecSym>,
            it: SpecTecIter,
            #[spectec_field(vec = true)]
            xes: Vec<SpecTecIterExp>,
        },
        #[spectec_node(name = "attr")]
        Attr { e: SpecTecExp, g1: Box<SpecTecSym> },
    }
}
pub use grammars::*;

mod premises {
    use crate::spectec::{MixOp, SpecTecExp, SpecTecIter, SpecTecIterExp};
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L164>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecPrem {
        #[spectec_node(name = "rule")]
        Rule { x: String, op: MixOp, e: SpecTecExp },
        #[spectec_node(name = "if")]
        If { e: SpecTecExp },
        #[spectec_node(name = "let")]
        Let { e1: SpecTecExp, e2: SpecTecExp },
        #[spectec_atom(name = "else")]
        Else,
        #[spectec_node(name = "iter")]
        Iter {
            pr1: Box<SpecTecPrem>,
            it: SpecTecIter,
            #[spectec_field(vec = true)]
            xes: Vec<SpecTecIterExp>,
        },
    }
}
pub use premises::*;

mod definitions {
    use crate::spectec::{MixOp, SpecTecDefTyp, SpecTecExp, SpecTecPrem, SpecTecSym, SpecTecTyp};
    use spectec_derive::SpecTecItem;

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L175>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecArg {
        #[spectec_node(name = "exp")]
        Exp { e: SpecTecExp },
        #[spectec_node(name = "typ")]
        Typ { t: SpecTecTyp },
        #[spectec_node(name = "def")]
        Def { x: String },
        #[spectec_node(name = "gram")]
        Gram { g: SpecTecSym },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L182>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecBind {
        #[spectec_node(name = "exp")]
        Exp { x: String, t: SpecTecTyp },
        #[spectec_node(name = "typ")]
        Typ { x: String },
        #[spectec_node(name = "def")]
        Def {
            x: String,
            #[spectec_field(vec = true)]
            ps: Vec<SpecTecParam>,
            t: SpecTecTyp,
        },
        #[spectec_node(name = "gram")]
        Gram {
            x: String,
            #[spectec_field(vec = true)]
            ps: Vec<SpecTecParam>,
            t: SpecTecTyp,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L189>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecParam {
        #[spectec_node(name = "exp")]
        Exp { x: String, t: SpecTecTyp },
        #[spectec_node(name = "typ")]
        Typ { x: String },
        #[spectec_node(name = "def")]
        Def {
            x: String,
            #[spectec_field(vec = true)]
            ps: Vec<SpecTecParam>,
            t: SpecTecTyp,
        },
        #[spectec_node(name = "gram")]
        Gram { x: String, t: SpecTecTyp },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L196>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecInst {
        #[spectec_node(name = "inst")]
        Inst {
            #[spectec_field(vec = true)]
            bs: Vec<SpecTecBind>,
            #[spectec_field(vec = true)]
            as_: Vec<SpecTecArg>,
            dt: SpecTecDefTyp,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L201>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecRule {
        #[spectec_node(name = "rule")]
        Rule {
            x: String,
            #[spectec_field(vec = true)]
            bs: Vec<SpecTecBind>,
            op: MixOp,
            e: SpecTecExp,
            #[spectec_field(vec = true)]
            prs: Vec<SpecTecPrem>,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L206>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecClause {
        #[spectec_node(name = "clause")]
        Clause {
            #[spectec_field(vec = true)]
            bs: Vec<SpecTecBind>,
            #[spectec_field(vec = true)]
            as_: Vec<SpecTecArg>,
            e: SpecTecExp,
            #[spectec_field(vec = true)]
            prs: Vec<SpecTecPrem>,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L211>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecProd {
        #[spectec_node(name = "prod")]
        Prod {
            #[spectec_field(vec = true)]
            bs: Vec<SpecTecBind>,
            g: SpecTecSym,
            e: SpecTecExp,
            #[spectec_field(vec = true)]
            prs: Vec<SpecTecPrem>,
        },
    }

    /// <https://github.com/WebAssembly/spec/blob/9479f1d0760494a93fcc73f7cf94c211ac91eec7/spectec/src/backend-ast/print.ml#L216>
    #[allow(unused)]
    #[derive(SpecTecItem, Debug, PartialEq)]
    pub enum SpecTecDef {
        #[spectec_node(name = "typ")]
        Typ {
            x: String,
            #[spectec_field(vec = true)]
            ps: Vec<SpecTecParam>,
            #[spectec_field(vec = true)]
            insts: Vec<SpecTecInst>,
        },
        #[spectec_node(name = "rel")]
        Rel {
            x: String,
            op: MixOp,
            t: SpecTecTyp,
            #[spectec_field(vec = true)]
            rules: Vec<SpecTecRule>,
        },
        #[spectec_node(name = "def")]
        Dec {
            x: String,
            #[spectec_field(vec = true)]
            ps: Vec<SpecTecParam>,
            t: SpecTecTyp,
            #[spectec_field(vec = true)]
            clauses: Vec<SpecTecClause>,
        },
        #[spectec_node(name = "gram")]
        Gram {
            x: String,
            #[spectec_field(vec = true)]
            ps: Vec<SpecTecParam>,
            t: SpecTecTyp,
            #[spectec_field(vec = true)]
            prods: Vec<SpecTecProd>,
        },
        #[spectec_node(name = "rec")]
        Rec {
            #[spectec_field(vec = true)]
            ds: Vec<SpecTecDef>,
        },
    }
}
pub use definitions::*;

/// Parses a SpecTec AST stream from the input string.
///
/// # Errors
///
/// Will return an error if any of the s-expressions cannot be decoded, or if the s-expressions are
/// not a valid SpecTec AST stream.
pub fn parse_spectec_stream(input: &str) -> Result<Vec<SpecTecDef>, crate::decode::DecodeError> {
    let sexpr_items = crate::sexpr::parse_sexpr_stream(input)?;
    let mut parsed = Vec::new();
    for item in sexpr_items {
        parsed.push(crate::decode::Decode::decode(item)?);
    }
    Ok(parsed)
}

#[cfg(test)]
mod test {
    use crate::decode::Decode;
    use crate::spectec::*;
    use spectec_derive::SpecTecItem;

    #[test]
    fn test_extra_string() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A { b: String },
        }

        let input = r#"(a "m" "c")"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_parse::spectec::test::test_extra_string::TestEnum: decoding variant A item not consumed by fields (A { b: \"m\" }): Unexpected item 'Text(\"c\")'"
            );
        }
    }

    #[test]
    fn test_extra_item() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<TestEnum>,
            },
        }

        let input = r#"(a (a) (a) (b))"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_parse::spectec::test::test_extra_item::TestEnum: decoding variant A item not consumed by fields (A { b: [A { b: [] }, A { b: [] }] }): Unexpected item 'Node(\"b\", [])'"
            );
        }
    }

    #[test]
    fn test_extra_item_unit() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A,
        }

        let input = r#"(a (b))"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_parse::spectec::test::test_extra_item_unit::TestEnum: decoding variant A unit type should have no items: Unexpected item 'Node(\"b\", [])'"
            );
        }
    }

    #[test]
    fn test_incompat_item() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<TestEnum>,
            },
        }

        let input = r#"(a "a")"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        for sexpr in sexprs {
            assert!(!TestEnum::can_decode(&sexpr));
            let parsed = TestEnum::decode(sexpr);
            assert!(parsed.is_err());
            assert_eq!(
                parsed.unwrap_err().to_string(),
                "while decoding spectec_parse::spectec::test::test_incompat_item::TestEnum: decoding variant A item not consumed by fields (A { b: [] }): Unexpected item 'Text(\"a\")'"
            );
        }
    }

    #[test]
    fn test_spectec_def_enum() {
        let input = r#"(typ "M" (inst (alias nat)))"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(SpecTecDef::can_decode(&sexpr));
                match SpecTecDef::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parsed,
            vec![SpecTecDef::Typ {
                x: "M".to_string(),
                ps: vec![],
                insts: vec![SpecTecInst::Inst {
                    bs: vec![],
                    as_: vec![],
                    dt: SpecTecDefTyp::Alias {
                        typ: SpecTecTyp::Num(SpecTecNumTyp::Nat),
                    },
                }]
            }]
        );
    }

    #[test]
    fn test_spectec_vec() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A {
                #[spectec_field(vec = true)]
                b: Vec<SpecTecUnOp>,
            },
        }

        let input = r#"(a minus minusplus not plus plusminus)"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parsed,
            vec![TestEnum::A {
                b: vec![
                    SpecTecUnOp::Minus,
                    SpecTecUnOp::MinusPlus,
                    SpecTecUnOp::Not,
                    SpecTecUnOp::Plus,
                    SpecTecUnOp::PlusMinus,
                ]
            }]
        );
    }

    #[test]
    fn test_spectec_atom_unnamed() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A { b: TestEnum2 },
        }
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum2 {
            #[spectec_atom()]
            C(TestEnum3),
        }
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum3 {
            #[spectec_atom(name = "d")]
            D,
        }

        let input = r#"(a d)"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };
        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(
            parsed,
            vec![TestEnum::A {
                b: TestEnum2::C(TestEnum3::D)
            }]
        );
    }

    #[test]
    fn test_spectec_node_unnamed_fields() {
        #[derive(SpecTecItem, Debug, PartialEq)]
        pub enum TestEnum {
            #[spectec_node(name = "a")]
            A(u64),
        }

        let input = r#"(a 0)"#;
        let sexprs = match crate::sexpr::parse_sexpr_stream(input) {
            Ok(p) => p,
            Err(e) => panic!("{}", e),
        };

        let parsed = sexprs
            .into_iter()
            .map(|sexpr| {
                assert!(TestEnum::can_decode(&sexpr));
                match TestEnum::decode(sexpr) {
                    Ok(p) => p,
                    Err(e) => panic!("{}", e),
                }
            })
            .collect::<Vec<_>>();
        assert_eq!(parsed, vec![TestEnum::A(0)]);
    }
}

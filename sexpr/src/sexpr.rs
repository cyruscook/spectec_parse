use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub enum SExprItem {
    Atom(String),
    Text(String),
    Node(String, Vec<SExprItem>),
}

impl Display for SExprItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SExprItem::Atom(s) => write!(f, "{s}"),
            SExprItem::Text(s) => write!(f, "\"{s}\""),
            SExprItem::Node(name, items) => {
                write!(f, "({name}")?;
                for item in items {
                    write!(f, " {item}")?;
                }
                write!(f, ")")
            }
        }
    }
}

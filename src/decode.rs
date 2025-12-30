use std::iter::Peekable;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DecodeError {
    #[error("Unrecognised symbol '{0}'")]
    UnrecognisedSymbol(String),
    #[error("Missing an expected item")]
    MissingItem,
    #[error("Unexpected item '{0:?}'")]
    UnexpectedItem(crate::sexpr::SExprItem),
    #[error("Error reading SExpr: {0}")]
    SExpr(#[from] crate::sexpr::SExprError),
    #[error("Error parsing a bool: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("Error parsing an int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{context}: {source}")]
    WithContext {
        #[source]
        source: Box<DecodeError>,
        context: String,
    },
}

impl DecodeError {
    #[must_use]
    pub fn with_context<R: AsRef<str>>(self, context: R) -> Self {
        Self::WithContext {
            source: Box::new(self),
            context: context.as_ref().to_string(),
        }
    }
}

pub(crate) trait Decode: Sized {
    /// Checks if the s-expression item can be decoded into Self.
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool;

    /// Decodes s-expression item into `Self`.
    ///
    /// # Errors
    ///
    /// Will return an error if the s-expression cannot be represented by `Self`. To avoid this case,
    /// use `can_decode` to check if the item can be decoded first.
    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError>;

    /// Decodes multiple items from an iterator into a `Vec<Self>`.
    /// Stops processing when an item is encountered that `T` cannot decode.
    fn decode_iter<I: Iterator<Item = crate::sexpr::SExprItem>>(
        items: &mut Peekable<I>,
    ) -> Result<Vec<Self>, DecodeError> {
        let mut parsed = Vec::new();
        while let Some(item) = items.peek() {
            if Self::can_decode(item) {
                // We know that there is another item as we've checked the result of peek
                #[allow(clippy::unwrap_used)]
                let item = items.next().unwrap();
                parsed.push(Self::decode(item)?);
            } else {
                // T cannot accept the item. Stop processing
                break;
            }
        }
        Ok(parsed)
    }
}

impl<T: Decode> Decode for Box<T> {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        T::can_decode(item)
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        Ok(Self::new(T::decode(item)?))
    }
}

impl<T: Decode> Decode for Option<T> {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        matches!(item, crate::sexpr::SExprItem::Text(_))
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        // https://ocaml.org/manual/5.4/api/Option.html#VALto_list
        match item {
            crate::sexpr::SExprItem::Text(t) => {
                if t == "[]" {
                    Ok(None)
                } else {
                    // Due to validation of length of t we know the arithmetic is safe
                    #[allow(clippy::arithmetic_side_effects)]
                    if t.len() > 2
                        && matches!(t.get(..=0), Some("["))
                        && matches!(t.get(t.len() - 1..), Some("]"))
                    {
                        Ok(Some(T::decode(crate::sexpr::SExprItem::Text(
                            t[1..t.len() - 1].to_owned(),
                        ))?))
                    } else {
                        Err(
                            DecodeError::UnexpectedItem(crate::sexpr::SExprItem::Text(t))
                                .with_context(format!(
                                    "while decoding {}",
                                    std::any::type_name::<Self>()
                                )),
                        )
                    }
                }
            }
            _ => Err(DecodeError::UnexpectedItem(item)
                .with_context(format!("while decoding {}", std::any::type_name::<Self>()))),
        }
    }
}

impl Decode for bool {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        matches!(item, crate::sexpr::SExprItem::Text(_))
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Atom(t) => Ok(t.parse()?),
            _ => Err(DecodeError::UnexpectedItem(item)
                .with_context(format!("while decoding {}", std::any::type_name::<Self>()))),
        }
    }
}

impl Decode for u64 {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        matches!(item, crate::sexpr::SExprItem::Text(_))
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Atom(t) => Ok(t.parse()?),
            _ => Err(DecodeError::UnexpectedItem(item)
                .with_context(format!("while decoding {}", std::any::type_name::<Self>()))),
        }
    }
}

impl Decode for i64 {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        matches!(item, crate::sexpr::SExprItem::Text(_))
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Atom(t) => Ok(t.parse()?),
            _ => Err(DecodeError::UnexpectedItem(item)
                .with_context(format!("while decoding {}", std::any::type_name::<Self>()))),
        }
    }
}

impl Decode for String {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        matches!(item, crate::sexpr::SExprItem::Text(_))
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Text(t) => Ok(t),
            _ => Err(DecodeError::UnexpectedItem(item)
                .with_context(format!("while decoding {}", std::any::type_name::<Self>()))),
        }
    }
}

/// Decodes multiple items from an iterator into a Vec<T>.
/// Stops processing when an item is encountered that T cannot decode.
/// This is a convenience wrapper around T::decode_iter, which can be used in macros to avoid the
/// need to specify T explicitly, instead the compiler can infer T from V (Vec<T>).
#[allow(unused, clippy::extra_unused_type_parameters)]
pub(crate) fn decode_iter<
    V: Into<Vec<T>>,
    T: Decode,
    I: Iterator<Item = crate::sexpr::SExprItem>,
>(
    items: &mut Peekable<I>,
) -> Result<Vec<T>, DecodeError> {
    T::decode_iter(items)
}

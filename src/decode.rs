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

    /// Takes from an iterator all the items that can be decoded into a Vec<T>.
    /// Always returns true, as we can decode the items we take from the iterator (even if none)
    fn can_decode_iter<'a, I: Iterator<Item = &'a crate::sexpr::SExprItem>>(
        items: &mut Peekable<I>,
    ) -> bool {
        while let Some(item) = items.peek() {
            if Self::can_decode(item) {
                // We know that there is another item as we've checked the result of peek
                #[allow(clippy::unwrap_used)]
                items.next().unwrap();
            } else {
                break;
            }
        }
        true
    }

    /// Decodes as many items from an iterator as we can.
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
                parsed.push(Self::decode(item).map_err(|e| {
                    e.with_context(format!("while decoding {}", std::any::type_name::<Self>()))
                })?);
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
        T::decode(item).map(Self::new).map_err(|e| {
            e.with_context(format!("while decoding {}", std::any::type_name::<Self>()))
        })
    }
}

impl Decode for bool {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        match item {
            crate::sexpr::SExprItem::Atom(t) => t.parse::<Self>().is_ok(),
            _ => false,
        }
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Atom(t) => t.parse().map_err(DecodeError::from),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

impl Decode for u64 {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        match item {
            crate::sexpr::SExprItem::Atom(t) => t.parse::<Self>().is_ok(),
            _ => false,
        }
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Atom(t) => t.parse().map_err(DecodeError::from),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

impl Decode for i64 {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        match item {
            crate::sexpr::SExprItem::Atom(t) => t.parse::<Self>().is_ok(),
            _ => false,
        }
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Atom(t) => t.parse().map_err(DecodeError::from),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

impl Decode for String {
    fn can_decode(item: &crate::sexpr::SExprItem) -> bool {
        matches!(item, crate::sexpr::SExprItem::Text(_))
    }

    fn decode(item: crate::sexpr::SExprItem) -> Result<Self, DecodeError> {
        match item {
            crate::sexpr::SExprItem::Text(t) => Ok(t),
            _ => Err(DecodeError::UnexpectedItem(item)),
        }
        .map_err(|e| e.with_context(format!("while decoding {}", std::any::type_name::<Self>())))
    }
}

/// Takes from an iterator one item, only if it could be decoded as T.
/// Always returns true.
#[allow(unused, clippy::extra_unused_type_parameters)]
pub(crate) fn can_decode_option<
    'a,
    V: From<Option<T>>,
    T: Decode,
    I: Iterator<Item = &'a crate::sexpr::SExprItem>,
>(
    items: &mut Peekable<I>,
) -> bool {
    if let Some(item) = items.peek()
        && T::can_decode(item)
    {
        // We know that there is another item as we've checked the result of peek
        #[allow(clippy::unwrap_used)]
        items.next().unwrap();
    }
    true
}

/// Takes from an iterator one item, only if it could be decoded as `T`, in which case the result
/// of decoding that item as `T` is returned, otherwise `Ok(None)` is returned.
#[allow(unused, clippy::extra_unused_type_parameters)]
pub(crate) fn decode_option<
    V: From<Option<T>>,
    T: Decode,
    I: Iterator<Item = crate::sexpr::SExprItem>,
>(
    items: &mut Peekable<I>,
) -> Result<Option<T>, DecodeError> {
    if let Some(item) = items.peek()
        && T::can_decode(item)
    {
        // We know that there is another item as we've checked the result of peek
        #[allow(clippy::unwrap_used)]
        return T::decode(items.next().unwrap()).map(Some);
    }
    Ok(None)
}

/// Takes from an iterator all the items that can be decoded into a Vec<T>.
/// Always returns true, as we can decode the items we take from the iterator (even if none)
/// This is a convenience wrapper around T::can_decode_iter, which can be used in macros to avoid
/// the need to specify T explicitly, instead the compiler can infer T from V (Vec<T>).
#[allow(unused, clippy::extra_unused_type_parameters)]
pub(crate) fn can_decode_iter<
    'a,
    V: Into<Vec<T>>,
    T: Decode,
    I: Iterator<Item = &'a crate::sexpr::SExprItem>,
>(
    items: &mut Peekable<I>,
) -> bool {
    T::can_decode_iter(items)
}

/// Decodes as many items from an iterator as we can.
/// Stops processing when an item is encountered that `T` cannot decode.
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

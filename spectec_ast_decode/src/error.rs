use std::{num::ParseIntError, str::ParseBoolError};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorKind {
    /*#[error("Unrecognised symbol '{0}'")]
    UnrecognisedSymbol(String),
    #[error("Missing an expected item")]
    MissingItem,
    #[error("Unexpected item '{0:?}'")]
    UnexpectedItem(sexpr::SExprItem),
    #[error("Error reading SExpr: {0}")]
    SExpr(#[from] sexpr::SExprError),*/
    #[error("Unrecognised atom symbol: {0}")]
    UnrecognisedAtomSymbol(String),
    #[error("Unrecognised node symbol: {0}")]
    UnrecognisedNodeSymbol(String),
    #[error("Required another S-expression but stream is empty")]
    RequiredMissingSExpr(),
    #[error("Could not decode from S-Expression: {0}")]
    CannotDecodeSExpr(String),
    #[error("Extra unparsed S-expression remaining: {0}")]
    UnparsedSExpr(String),
    #[error("Error parsing a bool: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("Error parsing an int: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Wrapped(#[from] Box<Error>),
}

#[derive(Error, Debug)]
pub struct Error {
    #[source]
    pub kind: ErrorKind,
    pub variant: Option<String>,
    pub field: Option<String>,
    pub decoding: &'static str,
}

impl Error {
    #[must_use]
    pub(crate) fn new<T: crate::Decode>(kind: ErrorKind) -> Self {
        Self {
            kind,
            variant: None,
            field: None,
            decoding: std::any::type_name::<T>(),
        }
    }

    #[must_use]
    pub fn with_variant<S: Into<String>>(mut self, variant: S) -> Self {
        self.variant = Some(variant.into());
        self
    }

    #[must_use]
    pub fn with_field<S: Into<String>>(mut self, field: S) -> Self {
        self.field = Some(field.into());
        self
    }

    #[must_use]
    pub fn unrecognised_atom_symbol<T: crate::Decode, S: Into<String>>(symbol: S) -> Self {
        Self::new::<T>(ErrorKind::UnrecognisedAtomSymbol(symbol.into()))
    }

    #[must_use]
    pub fn unrecognised_node_symbol<T: crate::Decode, S: Into<String>>(symbol: S) -> Self {
        Self::new::<T>(ErrorKind::UnrecognisedNodeSymbol(symbol.into()))
    }

    #[must_use]
    pub fn required_missing_sexpr<T: crate::Decode>() -> Self {
        Self::new::<T>(ErrorKind::RequiredMissingSExpr())
    }

    #[must_use]
    pub fn cannot_decode_sexpr<T: crate::Decode>(sexpr: &sexpr::SExprItem) -> Self {
        Self::new::<T>(ErrorKind::CannotDecodeSExpr(format!("{sexpr:?}")))
    }

    #[must_use]
    pub fn unparsed_sexpr<T: crate::Decode>(sexpr: &sexpr::SExprItem) -> Self {
        Self::new::<T>(ErrorKind::UnparsedSExpr(format!("{sexpr:?}")))
    }

    #[must_use]
    pub fn parse_bool_err<T: crate::Decode>(from: ParseBoolError) -> Self {
        Self::new::<T>(ErrorKind::from(from))
    }

    #[must_use]
    pub fn parse_int_err<T: crate::Decode>(from: ParseIntError) -> Self {
        Self::new::<T>(ErrorKind::from(from))
    }

    #[must_use]
    pub fn wrapped<T: crate::Decode>(from: Error) -> Self {
        Self::new::<T>(ErrorKind::from(Box::new(from)))
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error decoding {}", self.decoding)?;
        if let Some(variant) = &self.variant {
            write!(f, "::{variant}")?;
        }
        if let Some(field) = &self.field {
            write!(f, ".{field}")?;
        }
        write!(f, ": {}", self.kind)
    }
}

pub type Result<T> = core::result::Result<T, Error>;

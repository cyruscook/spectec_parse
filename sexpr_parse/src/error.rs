use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SExprError {
    #[error("io error at {position}: {source}")]
    Io {
        #[source]
        source: std::io::Error,
        position: usize,
    },
    #[error("utf8 error at {position}: {source}")]
    Utf8 {
        #[source]
        source: FromUtf8Error,
        position: usize,
    },
    #[error("unexpected byte at {position}: found {unexpected:#x}, expected {expected:#x}")]
    UnexpectedByte {
        unexpected: u8,
        expected: u8,
        position: usize,
    },
    #[error("{context}: {source}")]
    WithContext {
        #[source]
        source: Box<SExprError>,
        context: String,
    },
}

impl SExprError {
    #[must_use]
    pub fn with_context<R: AsRef<str>>(self, context: R) -> Self {
        Self::WithContext {
            source: Box::new(self),
            context: context.as_ref().to_string(),
        }
    }
}

use std::{error, fmt, result};

/// The error type for `Storage` operations.
#[derive(Debug)]
pub struct Error {
    repr: Repr,
}

impl Error {
    /// Returns the corresponding `ErrorKind` for this error.
    pub fn kind(&self) -> ErrorKind {
        match self.repr {
            Repr::Inner(k) => k,
            _ => ErrorKind::Other,
        }
    }

    pub(crate) fn new<E>(error: E) -> Self
    where
        E: Into<Box<dyn error::Error + Send + Sync>>,
    {
        Self {
            repr: Repr::Other(error.into()),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match &self.repr {
            Repr::Inner(_) => None,
            Repr::Other(src) => Some(src.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        self.repr.fmt(f)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            repr: Repr::Inner(kind),
        }
    }
}

#[derive(Debug)]
enum Repr {
    Inner(ErrorKind),
    Other(Box<dyn error::Error + 'static + Send + Sync>),
}

impl fmt::Display for Repr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> result::Result<(), fmt::Error> {
        match self {
            Repr::Inner(kind) => write!(f, "{}", kind.as_str()),
            Repr::Other(e) => e.fmt(f),
        }
    }
}

/// A list specifying categories of Storage error.
#[derive(Debug, Clone, Copy)]
pub enum ErrorKind {
    /// Active blob not set, often initialization failed.
    ActiveBlobNotSet,
    /// Input configuration is wrong.
    WrongConfig,
    /// Probably storage initialization failed.
    Uninitialized,
    /// Record not found
    RecordNotFound,
    /// Work directory is locked by another storage.
    /// Or the operation lacked the necessary privileges to complete.
    /// Stop another storage or delete `*.lock` file
    WorkDirInUse,
    /// Storage was initialized with different key size
    KeySizeMismatch,
    /// Any error not part of this list
    Other,
}

impl ErrorKind {
    fn as_str(self) -> &'static str {
        match self {
            ErrorKind::ActiveBlobNotSet => "active blob not set",
            ErrorKind::WrongConfig => "wrong config",
            ErrorKind::Uninitialized => "storage unitialized",
            ErrorKind::RecordNotFound => "record not found",
            ErrorKind::WorkDirInUse => "work dir in use",
            ErrorKind::KeySizeMismatch => "key size mismatch",
            ErrorKind::Other => "other",
        }
    }
}

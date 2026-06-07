use std::fmt;

/// Errors returned by the public API.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    EmptyFigure,
    InvalidSubplotIndex,
    InvalidFigureSize,
    RenderFailed(&'static str),
    Io(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::EmptyFigure => write!(f, "figure has no panels"),
            Error::InvalidSubplotIndex => write!(f, "invalid subplot index"),
            Error::InvalidFigureSize => write!(f, "figure size must be positive"),
            Error::RenderFailed(msg) => write!(f, "{msg}"),
            Error::Io(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

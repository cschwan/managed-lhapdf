use cxx::Exception;
use thiserror::Error;

/// Error struct that wraps all exceptions thrown by the LHAPDF library.
#[derive(Debug, Error)]
pub enum Error {
    /// Captures an exception coming from the C++ LHAPDF library.
    #[error(transparent)]
    LhapdfException(Exception),
    /// General error with a message.
    #[error("{0}")]
    General(String),
    /// Errors from within this library.
    #[error(transparent)]
    Other(anyhow::Error),
}

/// Type definition for results with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

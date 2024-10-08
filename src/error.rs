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
    /// A 404 'file not found' error when trying to download a file over HTTP.
    #[error("file not found")]
    Http404,
    /// Errors from within this library.
    #[error(transparent)]
    Other(anyhow::Error),
}

/// Type definition for results with an [`enum@Error`].
pub type Result<T> = std::result::Result<T, Error>;

impl From<Exception> for Error {
    fn from(err: Exception) -> Self {
        Self::LhapdfException(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::Other(anyhow::Error::new(err))
    }
}

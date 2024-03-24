use cxx::Exception;
use thiserror::Error;

/// Error struct that wraps all exceptions thrown by the LHAPDF library.
#[derive(Debug, Error)]
pub enum Error {
    /// Captures an exception coming from the C++ LHAPDF library.
    #[error(transparent)]
    LhapdfException(Exception),
    /// General error with a message
    #[error("{0}")]
    General(String),
}

/// Type definition for results with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

use cxx::Exception;
use thiserror::Error;

/// Error struct that wraps all exceptions thrown by the LHAPDF library.
#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    /// Captures an exception coming from the C++ LHAPDF library.
    LhapdfException(Exception),
}

/// Type definition for results with an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during authentication processes.
#[derive(Debug, Serialize)]
pub enum Error {
    DependencyError(String),

    RepositoryNotFound(String),

    GitBinaryNotFound(String),

    GitCloneFailed(String),

    GitDiffFailed(String),

    CleanupFailure(String),

    FileParseError(String),
}

impl From<crate::entities::Error> for Error {
    fn from(err: crate::entities::Error) -> Self {
        match err {
            _ => Error::DependencyError(err.to_string()),
        }
    }
}

impl From<crate::db::Error> for Error {
    fn from(err: crate::db::Error) -> Self {
        match err {
            _ => Error::DependencyError(err.to_string()),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

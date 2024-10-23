use derive_more::From;
use serde::Serialize;

/// A result type for upload processes.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during upload processes.
#[derive(Debug, Serialize, From)]
pub enum Error {
    RepositoryNotFound(String),

    CleanupFailure,

    FileParseError(String),

    UploadNotFound,

    UploadRejected,

    UploadFailed,

    UploadFailedCatastrophically,

    #[from]
    GitError(crate::git::Error),

    #[from]
    DatabaseError(crate::db::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

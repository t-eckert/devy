use derive_more::From;
use serde::Serialize;
use serde_json::Value;

/// A result type for Controller processes.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur during Controller processes.
#[derive(Debug, From, Serialize)]
pub enum Error {
    Generic(String),

    #[from]
    DatabaseError(crate::db::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

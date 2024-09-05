use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

/// A result type for posts actions.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur during Controller processes.
#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    Generic(String),

    #[from]
    DatabaseError(crate::db::Error),

    /// An error occurred while interacting with the database.
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        match val {
            _ => Self::Sqlx(val),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

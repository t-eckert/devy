use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur while performing an action on an entity.
#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    /// The requested entity was not found.
    EntityNotFound,

    /// The request was malformed.
    Malformed(String),

    /// A field was missing from the request.
    MissingField(String),

    /// An error occurred while interacting with the database.
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl Error {
    /// Returns an error indicating that the request was malformed.
    pub fn malformed(error: &str) -> Self {
        Self::Malformed(error.to_string())
    }

    /// Returns an error indicating that a field was missing from the request.
    pub fn missing_field(field: &str) -> Self {
        Self::MissingField(field.to_string())
    }
}

impl From<sqlx::Error> for Error {
    fn from(val: sqlx::Error) -> Self {
        match val {
            sqlx::Error::RowNotFound => Self::EntityNotFound,
            _ => Self::Sqlx(val),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

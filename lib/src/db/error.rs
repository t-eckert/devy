use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::{convert::Infallible, fmt::Debug};

/// A result type for database operations.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur while performing an operation with the database.
#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    /// The database configuration is invalid.
    ConfigurationError(String),

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
    /// Returns an error indicating that the database configuration is invalid.
    pub fn configuration_error(error: &str) -> Self {
        Self::ConfigurationError(error.to_string())
    }

    /// Returns an error indicating that the request was malformed.
    pub fn malformed(error: &str) -> Self {
        Self::Malformed(error.to_string())
    }

    /// Returns an error indicating that a field was missing from the request.
    pub fn missing_field(field: &str) -> Self {
        Self::MissingField(field.to_string())
    }
}

impl From<std::env::VarError> for Error {
    fn from(val: std::env::VarError) -> Self {
        Self::configuration_error(&val.to_string())
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

impl From<Infallible> for Error {
    fn from(_: Infallible) -> Self {
        Self::EntityNotFound
    }
}

impl From<serde_json::Error> for Error {
    fn from(val: serde_json::Error) -> Self {
        Self::Malformed(val.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

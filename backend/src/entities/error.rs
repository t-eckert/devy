use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    EntityNotFound,
    Malformed(String),
    MissingField(String),
    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl Error {
    pub fn malformed(error: &str) -> Self {
        Self::Malformed(error.to_string())
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

use serde::Serialize;
use std::fmt::Debug;

use crate::entities::error::Error as EntitiesError;

pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the `upload` module.
#[derive(Debug, Serialize)]
pub enum Error {
    Base,

    RepositoryNotFound(String),

    GitBinaryNotFound(String),

    GitCloneFailed(String),
}

impl From<EntitiesError> for Error {
    fn from(val: EntitiesError) -> Self {
        match val {
            _ => Self::Base,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

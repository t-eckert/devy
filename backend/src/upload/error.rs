use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use std::fmt::Debug;

use crate::entities::error::Error as EntitiesError;

pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the `upload` module.
#[derive(Debug, Serialize)]
pub enum Error {
    DependencyError(String),

    RepositoryNotFound(String),

    GitBinaryNotFound(String),

    GitCloneFailed(String),

    CleanupFailure(String),
}

impl From<EntitiesError> for Error {
    fn from(val: EntitiesError) -> Self {
        match val {
            _ => Self::DependencyError(format!("{:?}", val)),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

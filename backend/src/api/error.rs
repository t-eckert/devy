use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

use crate::entities::error::Error as EntitiesError;
use crate::upload::Error as UploadError;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    StatusCode(#[serde_as(as = "DisplayFromStr")] StatusCode),
}

impl From<StatusCode> for Error {
    fn from(val: StatusCode) -> Self {
        Self::StatusCode(val)
    }
}

impl From<EntitiesError> for Error {
    fn from(val: EntitiesError) -> Self {
        match val {
            EntitiesError::EntityNotFound => Self::StatusCode(StatusCode::NOT_FOUND),
            EntitiesError::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            EntitiesError::Sqlx(_) => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<UploadError> for Error {
    fn from(val: UploadError) -> Self {
        match val {
            UploadError::RepositoryNotFound(_) => Self::StatusCode(StatusCode::BAD_REQUEST),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::StatusCode(status) => status.into_response(),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

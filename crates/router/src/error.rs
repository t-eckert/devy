use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

use db::Error as DbError;
use entities::Error as EntitiesError;
use forms::Error as FormsError;
use upload::Error as UploadError;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum Error {
    StatusCode(#[serde_as(as = "DisplayFromStr")] StatusCode),
    ServeFailure,
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
            // EntitiesError::Sqlx(_) => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<UploadError> for Error {
    fn from(_: UploadError) -> Self {
        Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<DbError> for Error {
    fn from(_: DbError) -> Self {
        Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl From<FormsError> for Error {
    fn from(val: FormsError) -> Self {
        match val {
            FormsError::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
        }
    }
}

impl From<tokio::io::Error> for Error {
    fn from(_: tokio::io::Error) -> Self {
        Self::ServeFailure
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::StatusCode(status) => status.into_response(),
            Self::ServeFailure => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

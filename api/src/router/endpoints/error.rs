use axum::response::{IntoResponse, Response};
use http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

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

impl From<lib::entities::Error> for Error {
    fn from(val: lib::entities::Error) -> Self {
        match val {
            lib::entities::Error::EntityNotFound => Self::StatusCode(StatusCode::NOT_FOUND),
            lib::entities::Error::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            // EntitiesError::Sqlx(_) => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<crate::uploader::Error> for Error {
    fn from(err: crate::uploader::Error) -> Self {
        match err {
            crate::uploader::Error::RepositoryNotFound(_) => {
                Self::StatusCode(StatusCode::NOT_FOUND)
            }
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<crate::db::Error> for Error {
    fn from(err: crate::db::Error) -> Self {
        match err {
            crate::db::Error::EntityNotFound => Self::StatusCode(StatusCode::NOT_FOUND),
            crate::db::Error::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            crate::db::Error::MissingField { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<lib::forms::Error> for Error {
    fn from(val: lib::forms::Error) -> Self {
        match val {
            lib::forms::Error::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
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

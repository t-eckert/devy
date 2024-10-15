use axum::response::{IntoResponse, Response};
use derive_more::From;
use http::StatusCode;
use lib::{db, forms, uploader};
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

/// Result type for `routers`.
pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, From, Serialize)]
pub enum Error {
    StatusCode(#[serde_as(as = "DisplayFromStr")] StatusCode),

    ServeFailure,

    Generic(String),

    #[from]
    ControllerError(crate::controllers::Error),
}

impl From<StatusCode> for Error {
    fn from(val: StatusCode) -> Self {
        Self::StatusCode(val)
    }
}

impl From<uploader::Error> for Error {
    fn from(err: uploader::Error) -> Self {
        match err {
            uploader::Error::RepositoryNotFound(_) => Self::StatusCode(StatusCode::NOT_FOUND),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<db::Error> for Error {
    fn from(err: db::Error) -> Self {
        match err {
            db::Error::EntityNotFound => Self::StatusCode(StatusCode::NOT_FOUND),
            db::Error::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            db::Error::MissingField { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            _ => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

impl From<forms::Error> for Error {
    fn from(val: forms::Error) -> Self {
        match val {
            forms::Error::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            forms::Error::RequestFailed { .. } => {
                Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR)
            }
            forms::Error::Conflict { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
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

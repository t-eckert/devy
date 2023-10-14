use crate::entities::error::EntityError;
use http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

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

impl From<EntityError> for Error {
    fn from(val: EntityError) -> Self {
        match val {
            EntityError::NotFound => Self::StatusCode(StatusCode::NOT_FOUND),
            EntityError::Malformed { .. } => Self::StatusCode(StatusCode::BAD_REQUEST),
            EntityError::Sqlx(_) => Self::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

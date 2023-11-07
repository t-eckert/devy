use http::StatusCode;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

use crate::entities::error::Error as EntitiesError;

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
        }
    }
}

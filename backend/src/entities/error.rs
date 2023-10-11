use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};
use std::fmt::Debug;

#[serde_as]
#[derive(Debug, Serialize)]
pub enum EntityError {
    NotFound { entity: String },

    Malformed { error: String },

    Sqlx(#[serde_as(as = "DisplayFromStr")] sqlx::Error),
}

impl EntityError {
    pub fn not_found<T: Debug>(entity: T) -> Self {
        Self::NotFound {
            entity: format!("{:?}", entity),
        }
    }

    pub fn malformed(error: &str) -> Self {
        Self::Malformed {
            error: format!("{}", error),
        }
    }
}

impl From<sqlx::Error> for EntityError {
    fn from(val: sqlx::Error) -> Self {
        Self::Sqlx(val)
    }
}

impl core::fmt::Display for EntityError {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for EntityError {}

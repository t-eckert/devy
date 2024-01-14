use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::entities::error::Error as EntitiesError;
use crate::upload::Error as UploadError;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
/// Represents errors in form submission and processing.
pub enum Error {
    /// The input to a form is not properly formatted.
    Malformed { message: String },
}

impl From<EntitiesError> for Error {
    fn from(val: EntitiesError) -> Self {
        match val {
            EntitiesError::EntityNotFound => Self::Malformed {
                message: "Entity not found".to_string(),
            },
            EntitiesError::Malformed { .. } => Self::Malformed {
                message: "Malformed input".to_string(),
            },
            EntitiesError::Sqlx(_) => Self::Malformed {
                message: "Internal server error".to_string(),
            },
            _ => Self::Malformed {
                message: "Internal server error".to_string(),
            },
        }
    }
}

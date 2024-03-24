use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
/// Represents errors in form submission and processing.
pub enum Error {
    /// The input to a form is not properly formatted.
    Malformed { message: String },
}

impl From<db::Error> for Error {
    fn from(val: db::Error) -> Self {
        match val {
            db::Error::EntityNotFound => Self::Malformed {
                message: "Entity not found".to_string(),
            },
            db::Error::Malformed { .. } => Self::Malformed {
                message: "Malformed input".to_string(),
            },
            _ => Self::Malformed {
                message: "Internal server error".to_string(),
            },
        }
    }
}

impl From<entities::Error> for Error {
    fn from(val: entities::Error) -> Self {
        match val {
            entities::Error::EntityNotFound => Self::Malformed {
                message: "Entity not found".to_string(),
            },
            entities::Error::Malformed { .. } => Self::Malformed {
                message: "Malformed input".to_string(),
            },
            _ => Self::Malformed {
                message: "Internal server error".to_string(),
            },
        }
    }
}

use serde::Serialize;
use serde_with::serde_as;

pub type Result<T> = std::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize)]
/// Represents errors in form submission and processing.
pub enum Error {
    /// The input to a form is not properly formatted.
    Malformed { message: String },

    /// A request the form submission depends on failed.
    RequestFailed { message: String },

    /// There was a conflict with the current state of the system.
    Conflict { message: String },
}

impl From<crate::db::Error> for Error {
    fn from(val: crate::db::Error) -> Self {
        match val {
            crate::db::Error::EntityNotFound => Self::Malformed {
                message: "Entity not found".to_string(),
            },
            crate::db::Error::Malformed { .. } => Self::Malformed {
                message: "Malformed input".to_string(),
            },
            _ => Self::RequestFailed {
                message: "Internal server error".to_string(),
            },
        }
    }
}

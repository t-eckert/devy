use derive_more::From;
use jsonwebtoken;

/// A result type for JWT processes.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur during JWT processes.
#[derive(Debug, From)]
pub enum Error {
    JWTError(String),

    #[from]
    SerdeJson(serde_json::Error),

    #[from]
    JsonWebTokenError(jsonwebtoken::errors::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

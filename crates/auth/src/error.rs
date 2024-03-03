use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during authentication processes.
#[derive(Debug, Serialize)]
pub enum Error {
    /// The authentication configuration is invalid.
    ConfigurationError(String),

    /// GitHub returned an error when exchanging the code for a token.
    CodeExchangeForTokenFailed(String),

    /// GitHub returned an error when exchanging the token for a user.
    TokenExchangeForUserFailed(String),

    /// The GitHub user data could not be deserialized.
    UnableToDeserializeUser(String),
}

impl From<entities::error::Error> for Error {
    fn from(err: entities::error::Error) -> Self {
        match err {
            _ => Error::TokenExchangeForUserFailed(err.to_string()),
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(val: std::env::VarError) -> Self {
        Self::ConfigurationError(val.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

use serde::Serialize;

pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur during authentication processes.
#[derive(Debug, Serialize)]
pub enum Error {
    /// GitHub returned an error when exchanging the code for a token.
    CodeExchangeForTokenFailed(String),

    /// GitHub returned an error when exchanging the token for a user.
    TokenExchangeForUserFailed(String),

    /// The GitHub user data could not be deserialized.
    UnableToDeserializeUser(String),

    DependencyError(String),

    RepositoryNotFound(String),

    GitBinaryNotFound(String),

    GitCloneFailed(String),
    GitDiffFailed(String),

    CleanupFailure(String),

    FileParseError(String),
}

impl From<entities::Error> for Error {
    fn from(err: entities::Error) -> Self {
        Error::TokenExchangeForUserFailed(err.to_string())
    }
}

impl From<db::Error> for Error {
    fn from(err: db::Error) -> Self {
        Error::DependencyError(err.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

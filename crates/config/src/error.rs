pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when interacting with configuration.
#[derive(Debug)]
pub enum Error {
    /// A required environment variable was not found.
    MissingEnvironmentVariable,

    /// An error occurred when trying to parse an environment variable.
    EnvironmentVariableParseError,
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => Error::MissingEnvironmentVariable,
            std::env::VarError::NotUnicode(_) => Error::EnvironmentVariableParseError,
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}

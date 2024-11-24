use derive_more::From;

/// A result type for GitHub processes.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur during GitHub processes.
#[derive(Debug, From)]
pub enum Error {
    /// The authentication configuration is invalid.
    ConfigurationError(String),

    /// GitHub returned an error when exchanging the code for a token.
    CodeExchangeForTokenFailed,

    /// The GitHub user data could not be deserialized.
    UnableToDeserializeUser(String),

    /// An error occurred when interacting with the database.
    #[from]
    DatabaseError(crate::db::Error),

    #[from]
    SerdeJson(serde_json::Error),

    #[from]
    OpenSSLError(openssl::error::ErrorStack),

    #[from]
    ReqwestError(reqwest::Error),

    #[from]
    JwtError(jsonwebtoken::errors::Error),
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

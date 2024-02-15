pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when working with the database.
#[derive(Debug)]
pub enum Error {
    /// An environment variable is missing or misconfigured.
    EnvironmentVariableError(String),

    /// An error occurred while interfacing with SQL.
    SQLError(String),
}

impl Error {
    pub fn environment_variable_error(name: &str) -> Self {
        Error::EnvironmentVariableError(name.to_string())
    }

    pub fn sql_error(err: sqlx::Error) -> Self {
        Error::SQLError(err.to_string())
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<std::env::VarError> for Error {
    fn from(err: std::env::VarError) -> Self {
        match err {
            std::env::VarError::NotPresent => {
                Self::environment_variable_error("Missing environment variable")
            }
            std::env::VarError::NotUnicode(variable) => Self::environment_variable_error(&format!(
                "Environment variable misconfigured: {}",
                variable.into_string().unwrap_or("".to_string())
            )),
        }
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            _ => Self::sql_error(err),
        }
    }
}

impl std::error::Error for Error {}

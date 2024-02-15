pub type Result<T> = std::result::Result<T, Error>;

/// Errors that can occur when working with the database.
#[derive(Debug)]
pub enum Error {}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        match err {
            _ => unimplemented!(),
        }
    }
}

impl std::error::Error for Error {}

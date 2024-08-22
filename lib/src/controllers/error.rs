use derive_more::From;
use jsonwebtoken;
use jsonwebtoken::{DecodingKey, EncodingKey};
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// A result type for Controller processes.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur during Controller processes.
#[derive(Debug, From, Serialize)]
pub enum Error {
    #[from]
    DatabaseError(crate::db::Error)
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

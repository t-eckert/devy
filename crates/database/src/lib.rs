mod database;
mod error;

pub use database::{connect, Database};
pub use error::{Error, Result};

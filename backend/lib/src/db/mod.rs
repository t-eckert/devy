mod database;
mod error;

pub use database::{connect, Conn, DBConn, Database, MIGRATOR};
pub use error::{Error, Result};

pub mod follow;
pub mod like;
pub mod profile;
pub mod repo;
pub mod upload;
pub mod user;

pub mod maintenance;

pub struct Id {
    pub id: uuid::Uuid,
}

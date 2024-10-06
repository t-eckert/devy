mod database;
mod error;

pub use database::{connect, Conn, DBConn, Database, MIGRATOR};
pub use error::{Error, Result};

pub mod user;

pub mod maintenance;

pub struct Id {
    pub id: uuid::Uuid,
}

mod config;
mod database;
mod error;

pub use config::Config;
pub use database::{connect, Database};
pub use error::{Error, Result};

pub mod blog;
pub mod entry;
pub mod feed;
pub mod feed_config;
pub mod like;
pub mod post;
pub mod profile;
pub mod repo;
pub mod session;
pub mod upload;
pub mod user;
pub mod webhook;

pub async fn migrate(config: Config) -> Result<()> {
    let db = connect(config).await?;
    sqlx::migrate!("./migrations")
        .run(&db)
        .await
        .map_err(|_| Error::configuration_error("Unable to run migrations"))?;
    Ok(())
}

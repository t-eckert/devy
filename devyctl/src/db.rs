use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type DB = Pool<Postgres>;

pub async fn connect(database_url: &str) -> Result<DB, crate::Error> {
    PgPoolOptions::new()
        .max_connections(100)
        .max_lifetime(Duration::from_secs(30 * 60))
        .connect(database_url)
        .await
        .map_err(|err| crate::Error::ConnectingToDatabase(err.to_string()))
}

use crate::db::Result;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

pub type Database = sqlx::PgPool;
pub type DBConn = sqlx::PgPool;

/// Connect to the database and run migrations.
/// Returns a connection to the database.
pub async fn connect(database_url: &str) -> Result<DBConn> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;
    MIGRATOR.run(&pool).await.unwrap();

    Ok(pool)
}

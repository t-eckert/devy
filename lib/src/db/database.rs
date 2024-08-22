use crate::db::Result;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

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
    sqlx::migrate!().run(&pool).await.unwrap();

    Ok(pool)
}

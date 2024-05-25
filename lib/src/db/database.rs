use crate::db::Result;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub type Database = sqlx::PgPool;

pub async fn connect(database_url: &str) -> Result<Database> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;
    sqlx::migrate!().run(&pool).await.unwrap();

    Ok(pool)
}

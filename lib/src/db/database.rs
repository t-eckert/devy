use crate::db::Config;
use crate::db::Result;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub type Database = sqlx::PgPool;

pub async fn connect(config: Config) -> Result<Database> {
    let connection_string = config.database_url;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(&connection_string)
        .await?;
    sqlx::migrate!().run(&pool).await.unwrap();

    Ok(pool)
}

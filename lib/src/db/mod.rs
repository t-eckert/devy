pub mod maintenance;

mod error;

pub use error::{Error, Result};

use sqlx::postgres::{PgListener, PgPoolOptions};
use std::time::Duration;

/// A migrator that can be referenced from other crates.
pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!();

/// A connection to the Postgres database.
pub type Conn = sqlx::PgPool;

/// A listener for Postgres notifications.
pub type Listener = PgListener;

/// An Id struct for holding unique identifiers of entities.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Id {
    pub id: uuid::Uuid,
}

/// Connect to the database and run migrations.
/// Update all posts with their like count.
/// Returns a connection to the database.
pub async fn connect(database_url: &str) -> Result<Conn> {
    let db_conn = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(30))
        .connect(database_url)
        .await?;
    MIGRATOR.run(&db_conn).await.unwrap();
    maintenance::update_all_posts_with_like_count(&db_conn)
        .await
        .unwrap();

    Ok(db_conn)
}

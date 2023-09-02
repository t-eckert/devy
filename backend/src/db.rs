use rocket::{fairing, Build, Rocket};
use rocket_db_pools::Database;

use sqlx;

#[derive(Database)]
#[database("postgres")]
pub struct DB(sqlx::PgPool);

/// Run database migrations on startup.
pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match DB::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },
        None => Err(rocket),
    }
}

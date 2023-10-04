use crate::auth::Client;
use sqlx::PgPool;

/// Store is the shared state of the application.
/// It contains a pool of Postgres connections and a client for handling authentication.
#[derive(Clone)]
pub struct Store {
    pub pool: PgPool,
    pub auth_client: Client,
}

impl Store {
    pub fn new(pool: PgPool, auth_client: Client) -> Self {
        Store { pool, auth_client }
    }
}

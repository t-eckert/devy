use crate::auth::Client;
use crate::uploader::Uploader;
use sqlx::PgPool;

/// Store is the shared state of the application.
/// It contains a pool of Postgres connections and a client for handling authentication.
#[derive(Clone)]
pub struct Store {
    pub pool: PgPool,
    pub auth_client: Client,
    pub uploader: Uploader,
}

impl Store {
    pub fn new(pool: PgPool, auth_client: Client, uploader: Uploader) -> Self {
        Self {
            pool,
            auth_client,
            uploader,
        }
    }
}

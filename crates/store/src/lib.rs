use auth::Client;
use db::Database;

/// Store is the shared state of the application.
/// It contains a database pool, an authentication client, and an uploader.
#[derive(Clone)]
pub struct Store {
    pub db: Database,
    pub auth_client: Client,
}

impl Store {
    pub fn new(db: Database, auth_client: Client) -> Self {
        Self { db, auth_client }
    }
}

use auth::{Client, Provider};
use db::Database;

/// Store is the shared state of the application.
/// It contains a database pool, configuration, an authentication client, and an uploader.
#[derive(Clone)]
pub struct Store<P> {
    pub db: Database,
    pub auth_client: Client<P>,
}

impl<P> Store<P>
where
    P: Provider,
{
    pub fn new(db: Database, auth_client: Client<P>) -> Self {
        Self { db, auth_client }
    }
}

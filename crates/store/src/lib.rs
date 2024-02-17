use database::Database;

/// Store is the shared state of the application.
/// It contains a database pool, configuration, an authentication client, and an uploader.
#[derive(Clone)]
pub struct Store {
    pub db: Database,
}

use crate::auth::Client as AuthClient;
use crate::github::Client as GitHubClient;
use lib::db;

/// Store is the shared state of the application.
/// It contains a database pool, an authentication client, and an uploader.
#[derive(Clone, Debug)]
pub struct Store {
    /// The database connection pool.
    pub db_conn: db::Conn,
    /// The authentication client.
    pub auth_client: AuthClient,
    /// The GitHub client.
    pub github_client: GitHubClient,
}

impl Store {
    /// Create a new Store.
    pub fn new(db_conn: db::Conn, auth_client: AuthClient, github_client: GitHubClient) -> Self {
        Self {
            db_conn,
            auth_client,
            github_client,
        }
    }
}

use crate::auth::Client as AuthClient;
use crate::db::Database;
use crate::github::Client as GitHubClient;
use crate::uploader::Uploader;

/// Store is the shared state of the application.
/// It contains a database pool, an authentication client, and an uploader.
#[derive(Clone)]
pub struct Store {
    pub db: Database,
    pub auth_client: AuthClient,
    pub uploader: Uploader,
    pub github_client: GitHubClient,
}

impl Store {
    pub fn new(
        db: Database,
        auth_client: AuthClient,
        uploader: Uploader,
        github_client: GitHubClient,
    ) -> Self {
        Self {
            db,
            auth_client,
            uploader,
            github_client,
        }
    }
}

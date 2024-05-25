use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct GitHubClient {
    app_id: String,
    private_key: String,
}

impl GitHubClient {
    pub fn new(app_id: &str, private_key: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            private_key: private_key.to_string(),
        }
    }

    pub async fn fetch_user_installations(&self) -> Result<HashSet<String>, reqwest::Error> {
        let users = HashSet::new();

        Ok(users)
    }
}

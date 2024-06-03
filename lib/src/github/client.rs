use crate::jwt::{Subject, JWT};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct GitHubClient {
    app_id: String,
    private_key: String,
}

#[derive(Deserialize, Debug)]
struct Installation {
    id: u64,
    account: Account,
}

#[derive(Deserialize, Debug)]
struct Account {
    login: String,
}

impl GitHubClient {
    pub fn new(app_id: &str, private_key: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            private_key: private_key.to_string(),
        }
    }

    pub async fn fetch_user_installations(&self) -> Result<HashSet<String>, reqwest::Error> {
        dbg!("Fetching user installations");

        let users = HashSet::new();

        let jwt = JWT::new(self.private_key.clone()).unwrap();
        let token = jwt
            .encode(Subject::AuthToken, serde_json::json!({"iss": self.app_id}))
            .unwrap();

        println!("{token}");

        let client = Client::new();
        let response = client
            .get("https://api.github.com/app/installations")
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .header(ACCEPT, "application/vnd.github+json")
            .send()
            .await?;
        dbg!(&response);

        let installations: Vec<Installation> = response.json().await?;
        dbg!(&installations);
        Ok(users)
    }
}

use super::error::Error;
use jsonwebtoken::encode;
use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::Client as ReqwestClient;
use serde::Deserialize;
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Client {
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

impl Client {
    /// Create a new GitHub client.
    pub fn new(app_id: &str, private_key: &str) -> Self {
        Self {
            app_id: app_id.to_string(),
            private_key: private_key.to_string(),
        }
    }

    /// Fetch the installations for the user.
    pub async fn fetch_user_installations(&self) -> Result<HashSet<String>, Error> {
        dbg!("Fetching user installations");

        let users = HashSet::new();

        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &serde_json::json!({
                "iss": self.app_id,
            }),
            &jsonwebtoken::EncodingKey::from_rsa_pem(self.private_key.as_bytes()).unwrap(),
        )?;

        println!("{token}");

        let client = ReqwestClient::new();
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

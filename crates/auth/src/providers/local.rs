use std::collections::HashMap;

use crate::error::Result;
use db::Database;

#[derive(Clone)]
pub struct LocalProvider {
    redirect_url: String,
}

impl LocalProvider {
    pub fn new(redirect_url: String) -> Self {
        Self { redirect_url }
    }

    pub fn login(&self) -> String {
        println!("Logging in with GitHub");

        String::from("asdf")
    }

    pub fn logout(&self) {
        println!("Logging out from GitHub");
    }

    pub async fn handle_callback(&self, db: Database, params: HashMap<String, String>) -> String {
        println!("Handling local callback");
        "http://localhost:3000".to_string()
    }

    pub fn validate_session(&self, _db: Database, _session: &str) -> Result<()> {
        println!("Validating local session");
        Ok(())
    }
}

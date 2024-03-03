use crate::error::Result;
use db::Database;

#[derive(Clone)]
pub struct LocalProvider {}

impl LocalProvider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn login(&self) {
        println!("Logging in with GitHub");
    }

    pub fn logout(&self) {
        println!("Logging out from GitHub");
    }

    pub fn handle_callback(&self) -> Result<()> {
        println!("Handling local callback");
        Ok(())
    }

    pub fn validate_session(&self, _session: &str, _db: Database) -> Result<()> {
        println!("Validating local session");
        Ok(())
    }
}

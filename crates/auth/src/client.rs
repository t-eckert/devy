use crate::{error::Result, Providers};
use db::Database;

#[derive(Clone)]
pub struct Client {}

impl Client {
    pub fn new(provider: Providers) -> Self {
        Self {}
    }

    pub fn login(&self) {}

    pub fn logout(&self) {}

    pub fn handle_callback(&self) -> Result<()> {
        Ok(())
    }

    pub fn validate_session(&self, _session: &str, _db: Database) -> Result<()> {
        Ok(())
    }
}

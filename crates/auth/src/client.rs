use crate::{error::Result, Provider};
use db::Database;

#[derive(Debug, Clone)]
pub struct Client<P> {
    backend: P,
}

impl<P> Client<P>
where
    P: Provider,
{
    pub fn new(backend: P) -> Self {
        Self { backend }
    }

    pub fn login(&self) {
        self.backend.login();
    }

    pub fn logout(&self) {
        self.backend.logout();
    }

    pub fn handle_callback(&self) -> Result<()> {
        self.backend.handle_callback()
    }

    pub fn validate_session(&self, _session: &str, _db: Database) -> Result<()> {
        Ok(())
    }
}

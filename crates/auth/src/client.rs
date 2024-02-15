use crate::{backend::Backend, error::Result};
use database::Database;

pub struct Client<B> {
    backend: B,
}

impl<B> Client<B>
where
    B: Backend,
{
    pub fn new(backend: B) -> Self {
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

    pub fn validate_session(&self, session: &str, db: Database) -> Result<()> {
        Ok(())
    }
}

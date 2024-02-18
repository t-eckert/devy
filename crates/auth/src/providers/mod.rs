mod github;
mod local;

use crate::error::Result;

pub use github::GitHubProvider;
pub use local::LocalProvider;

pub trait Provider {
    fn login(&self) {}
    fn logout(&self) {}
    fn handle_callback(&self) -> Result<()> {
        Ok(())
    }
}

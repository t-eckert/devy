mod github;
mod local;

use crate::error::Result;

pub use github::GitHubBackend;
pub use local::LocalBackend;

pub trait Backend {
    fn login(&self) {}
    fn logout(&self) {}
    fn handle_callback(&self) -> Result<()> {
        Ok(())
    }
}

use crate::error::Result;
use crate::Provider;
use oauth2::basic::BasicClient;

pub struct GitHubProvider {
    _oauth_client: BasicClient,
    _callback_url: String,
}

impl Provider for GitHubProvider {
    fn login(&self) {
        println!("Logging in with GitHub");
    }

    fn logout(&self) {
        println!("Logging out from GitHub");
    }

    fn handle_callback(&self) -> Result<()> {
        Ok(())
    }
}

use std::collections::HashMap;

use crate::store::Store;

use super::Result;
use lib::auth::Provider;
use serde::Deserialize;

pub struct AuthController;

#[derive(Debug, Deserialize)]
pub struct NewLogin {
    pub provider: Provider,
    pub code: String,
    pub success_url: String,
    pub error_url: String,
}

impl AuthController {
    pub async fn sign_in(store: &Store, login: &NewLogin) -> Result<String> {
        Ok("/".to_string())
    }

    pub async fn providers(store: &Store) -> Result<HashMap<Provider, String>> {
        let mut providers = HashMap::new();
        providers.insert(
            Provider::GitHub,
            store.auth_client.provider_auth_url(Provider::GitHub),
        );

        Ok(providers)
    }
}

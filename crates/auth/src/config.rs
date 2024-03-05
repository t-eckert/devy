use crate::error::{Error, Result};
use crate::providers::Provider;
use std::env;

pub struct Config {
    pub provider: Provider,
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: String,
    pub redirect_url: String,
}

impl Config {
    pub fn new(
        provider: Provider,
        client_id: String,
        client_secret: String,
        callback_url: String,
        redirect_url: String,
    ) -> Self {
        Self {
            provider,
            client_id,
            client_secret,
            callback_url,
            redirect_url,
        }
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self {
            provider: parse_provider(env::var("AUTH_PROVIDER")?)?,
            client_id: env::var("CLIENT_ID")?,
            client_secret: env::var("CLIENT_SECRET")?,
            callback_url: env::var("CALLBACK_URL")?,
            redirect_url: env::var("REDIRECT_URL")?,
        })
    }
}

fn parse_provider(var: String) -> Result<Provider> {
    match var.as_str() {
        "github" => Ok(Provider::GitHub),
        "local" => Ok(Provider::Local),
        _ => Err(Error::ConfigurationError("Invalid provider".to_string())),
    }
}

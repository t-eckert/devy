use super::error::Result;
use std::env;

pub struct Config {
    pub client_id: String,
    pub client_secret: String,
    pub callback_url: String,
    pub redirect_url: String,
    pub encoding_key: String,
}

impl Config {
    pub fn new(
        client_id: String,
        client_secret: String,
        callback_url: String,
        redirect_url: String,
        encoding_key: String,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            callback_url,
            redirect_url,
            encoding_key,
        }
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self {
            client_id: env::var("CLIENT_ID")?,
            client_secret: env::var("CLIENT_SECRET")?,
            callback_url: env::var("CALLBACK_URL")?,
            redirect_url: env::var("REDIRECT_URL")?,
            encoding_key: env::var("ENCODING_KEY")?,
        })
    }
}

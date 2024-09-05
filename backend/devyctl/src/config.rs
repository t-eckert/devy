const DATABASE_URL: &str = "DATABASE_URL";
const ENCODING_KEY: &str = "ENCODING_KEY";
const GITHUB_APP_CLIENT_ID: &str = "GITHUB_APP_CLIENT_ID";
const GITHUB_APP_CLIENT_SECRET: &str = "GITHUB_APP_CLIENT_SECRET";
const GITHUB_APP_PRIVATE_KEY: &str = "GITHUB_APP_PRIVATE_KEY";

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String),
}

// Keep these fields in alphabetical order.
pub struct Config {
    pub database_url: String,
    pub encoding_key: String,
    pub github_app_client_id: String,
    pub github_app_client_secret: String,
    pub github_app_private_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env(DATABASE_URL)?,
            encoding_key: env(ENCODING_KEY)?,
            github_app_client_id: env(GITHUB_APP_CLIENT_ID)?,
            github_app_client_secret: env(GITHUB_APP_CLIENT_SECRET)?,
            github_app_private_key: env(GITHUB_APP_PRIVATE_KEY)?,
        })
    }
}

fn env(key: &str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::MissingEnv(key.to_owned()))
}

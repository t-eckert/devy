const CALLBACK_URL: &str = "CALLBACK_URL";
const DATABASE_URL: &str = "DATABASE_URL";
const ENCODING_PRIVATE_KEY: &str = "ENCODING_PRIVATE_KEY";
const ENCODING_PUBLIC_KEY: &str = "ENCODING_PUBLIC_KEY";
const GITHUB_APP_CLIENT_ID: &str = "GITHUB_APP_CLIENT_ID";
const GITHUB_APP_CLIENT_SECRET: &str = "GITHUB_APP_CLIENT_SECRET";
const GITHUB_APP_PRIVATE_KEY: &str = "GITHUB_APP_PRIVATE_KEY";
const REDIRECT_URL: &str = "REDIRECT_URL";

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ConfigError::MissingEnv(key) => write!(f, "Missing environment variable: {}", key),
        }
    }
}

impl std::error::Error for ConfigError {}

// Keep these fields in alphabetical order.
pub struct Config {
    pub callback_url: String,
    pub database_url: String,
    pub encoding_private_key: String,
    pub encoding_public_key: String,
    pub github_app_client_id: String,
    pub github_app_client_secret: String,
    pub github_app_private_key: String,
    pub redirect_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(Config {
            callback_url: env(CALLBACK_URL)?,
            database_url: env(DATABASE_URL)?,
            encoding_private_key: env(ENCODING_PRIVATE_KEY)?,
            encoding_public_key: env(ENCODING_PUBLIC_KEY)?,
            github_app_client_id: env(GITHUB_APP_CLIENT_ID)?,
            github_app_client_secret: env(GITHUB_APP_CLIENT_SECRET)?,
            github_app_private_key: env(GITHUB_APP_PRIVATE_KEY)?,
            redirect_url: env(REDIRECT_URL)?,
        })
    }
}

fn env(key: &str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::MissingEnv(key.to_owned()))
}

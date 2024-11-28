const OPENAI_API_KEY: &str = "OPENAI_API_KEY";

#[derive(Debug)]
pub enum ConfigError {
    MissingEnv(String),
}

// Keep these fields in alphabetical order.
#[derive(Debug)]
pub struct Config {
    pub openai_api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenvy::dotenv().ok();

        Ok(Config {
            openai_api_key: env(OPENAI_API_KEY)?,
        })
    }
}

fn env(key: &str) -> Result<String, ConfigError> {
    std::env::var(key).map_err(|_| ConfigError::MissingEnv(key.to_owned()))
}

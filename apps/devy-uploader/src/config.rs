use crate::Error;

const DATABASE_URL: &str = "DATABASE_URL";
const GIT_PATH: &str = "GIT_PATH";

// Keep these fields in alphabetical order.
pub struct Config {
    pub database_url: String,
    pub git_path: String,
}

impl Config {
    pub fn from_env() -> Result<Self, Error> {
        dotenvy::dotenv().ok();

        Ok(Config {
            database_url: env(DATABASE_URL)?,
            git_path: env(GIT_PATH)?,
        })
    }
}

fn env(key: &str) -> Result<String, Error> {
    std::env::var(key).map_err(|_| Error::ConfigError(key.to_owned()))
}

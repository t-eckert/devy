use crate::error::Result;
use std::env;

/// The configuration for the database.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
}

impl Config {
    pub fn new(database_url: String) -> Self {
        Self { database_url }
    }

    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_env() {
        env::set_var("DATABASE_URL", "test");
        assert_eq!(Config::from_env().unwrap().database_url, "test");
    }
}

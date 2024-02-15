mod config;
mod error;

pub use config::Config;
pub use error::{Error, Result};

pub fn load_dotenv() {
    let _ = dotenvy::dotenv();
}

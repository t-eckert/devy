use std::fs;
use tracing::{event, Level};

use super::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Git {
    bin: String,
}

impl Git {
    pub fn new(bin: String) -> Result<Self> {
        if !fs::metadata(&bin).is_ok() {
            return Err(Error::GitBinaryNotFound(format!(
                "Git binary not found at {}",
                bin
            )));
        }

        Ok(Self { bin })
    }

    pub fn clone_repo(self, dir: &str, url: &str) -> Result<()> {
        let output = std::process::Command::new(&self.bin)
            .arg("clone")
            .arg(url)
            .arg(dir)
            .output()
            .map_err(|_| Error::GitCloneFailed(format!("Failed to clone repo {}", url)))?;

        event!(Level::INFO, "Git clone output: {:?}", output);

        Ok(())
    }
}

use std::fs;

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

    pub fn clone_repo(&self, dir: &str, url: &str) -> Result<()> {
        std::process::Command::new(&self.bin)
            .arg("clone")
            .arg(url)
            .arg(dir)
            .output()
            .map_err(|_| Error::GitCloneFailed(format!("Failed to clone repo {}", url)))?;

        Ok(())
    }

    pub fn sha(&self, dir: &str) -> Result<String> {
        let output = std::process::Command::new(&self.bin)
            .arg("rev-parse")
            .arg("HEAD")
            .current_dir(dir)
            .output()
            .map_err(|_| Error::GitDiffFailed("Failed to get sha".to_string()))?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn first_sha(&self, dir: &str) -> Result<String> {
        let output = std::process::Command::new(&self.bin)
            .arg("rev-list")
            .arg("--max-parents=0")
            .arg("HEAD")
            .current_dir(dir)
            .output()
            .map_err(|_| Error::GitDiffFailed("Failed to get sha".to_string()))?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    pub fn diff(&self, dir: &str, to: &str, from: &str) -> Result<String> {
        let output = std::process::Command::new(&self.bin)
            .arg("--no-pager")
            .arg("diff")
            .arg(from)
            .arg(to)
            .arg("--name-status")
            .current_dir(dir)
            .output()
            .map_err(|_| Error::GitDiffFailed("Failed to diff".to_string()))?;

        dbg!(&output);

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

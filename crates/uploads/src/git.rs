use crate::error::{Error, Result};
use crate::uploader::Diff;
use std::fs;

/// A struct that represents a Git binary.
#[derive(Debug, Clone)]
pub struct Git {
    bin: String,
}

impl Git {
    /// Create a new Git instance.
    pub fn new(bin: String) -> Result<Self> {
        if !fs::metadata(&bin).is_ok() {
            return Err(Error::GitBinaryNotFound(format!(
                "Git binary not found at {}",
                bin
            )));
        }

        Ok(Self { bin })
    }

    /// Clone the repo at the url to the given directory.
    pub fn clone_repo(&self, dir: &str, url: &str) -> Result<()> {
        std::process::Command::new(&self.bin)
            .arg("clone")
            .arg(url)
            .arg(dir)
            .output()
            .map_err(|_| Error::GitCloneFailed(format!("Failed to clone repo {}", url)))?;

        Ok(())
    }

    /// Get the SHA of the HEAD commit in the given directory.
    pub fn sha(&self, dir: &str) -> Result<String> {
        let output = std::process::Command::new(&self.bin)
            .arg("rev-parse")
            .arg("HEAD")
            .current_dir(dir)
            .output()
            .map_err(|_| Error::GitDiffFailed("Failed to get sha".to_string()))?;

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Get the SHA of the first commit in the given directory.
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

    /// Get the diff between two commits in the given directory.
    pub fn diff(&self, dir: &str, to: &str, from: &str) -> Result<Vec<Diff>> {
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

        Ok(Diff::from_raw(
            String::from_utf8_lossy(&output.stdout).to_string(),
        ))
    }
}

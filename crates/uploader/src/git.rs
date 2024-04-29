use crate::error::{Error, Result};
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> String {
        let output = std::process::Command::new("which")
            .arg("git")
            .output()
            .unwrap();
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    #[test]
    fn test_git_new() {
        let binary_path = setup();
        let git = Git::new(binary_path);
        assert!(git.is_ok());
    }

    #[test]
    fn test_git_behaviors() {
        let binary_path = setup();
        let dir = "/tmp/test";

        let git = Git::new(binary_path).unwrap();

        // Clone
        let clone = git.clone_repo(&dir, &"https://github.com/t-eckert/devylog");
        assert!(clone.is_ok());
        let ls = std::process::Command::new("ls").current_dir(dir).output();
        assert!(ls.is_ok());

        // SHA
        let sha = git.sha(dir);
        assert!(sha.is_ok());

        // First SHA
        let first_sha = git.first_sha(dir);
        assert!(first_sha.is_ok());

        // Diff
        let diff = git.diff(dir, &sha.unwrap(), &first_sha.unwrap());
        assert!(diff.is_ok());

        // Cleanup
        std::process::Command::new("rm")
            .arg("-rf")
            .arg(dir)
            .output()
            .unwrap();
    }
}

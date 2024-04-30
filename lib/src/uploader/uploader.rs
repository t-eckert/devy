use super::{
    cleanup::cleanup, clone::clone, diff::diff, error::Result, sync::sync, verify::verify, Git,
};
use crate::db::Database;
use crate::entities::Upload;

/// The Uploader struct is responsible for handling the upload process.
#[derive(Clone)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    pub async fn upload(self, db: &Database, mut upload: Upload) -> Result<Upload> {
        verify(db, &mut upload).await?;
        clone(db, &mut upload, &self.git).await?;
        let diff = diff(db, &upload, &self.git).await?;
        sync(db, &mut upload, diff).await;
        cleanup(&mut upload);

        Ok(upload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Git {
        let output = std::process::Command::new("which")
            .arg("git")
            .output()
            .unwrap();
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Git::new(path).unwrap()
    }

    #[tokio::test]
    async fn test_upload() {
        let git = setup();

        let uploader = Uploader::new(git);
    }
}

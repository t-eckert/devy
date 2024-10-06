use super::{sync, Error, Result, Upload, UploadRepository};
use crate::db::Database;
use crate::git::Git;
use std::collections::HashMap;
use uuid::Uuid;

/// The Uploader struct is responsible for handling the upload process.
#[derive(Clone)]
pub struct Uploader {
    git: Git,
    uploads: HashMap<Uuid, Upload>,
}

impl Uploader {
    pub fn new(git: Git) -> Self {
        Self {
            git,
            uploads: HashMap::new(),
        }
    }

    pub async fn upload(mut self, db_conn: &Database, mut upload: Upload) -> Result<Upload> {
        let id = upload.id;

        self.uploads.insert(upload.id, upload);

        let upload = self
            .setup(db_conn, id)
            .await?
            .sync(db_conn, id)
            .await?
            .cleanup(id)?
            .uploads
            .remove(&id)
            .ok_or(Error::UploadNotFound)?
            .to_owned();

        Ok(upload)
    }

    // setup verifies the upload, inserts it into the database if valid, then clones the repository associated with the upload.
    async fn setup(mut self, db_conn: &Database, id: Uuid) -> Result<Self> {
        let upload = self
            .uploads
            .remove(&id)
            .ok_or(Error::UploadNotFound)?
            .to_owned();

        let upload = upload.verify(db_conn).await?.receive(db_conn).await?;
        let upload = cof(&upload.dir(), upload.clone_repo(db_conn, &self.git).await)?;

        self.uploads.insert(id, upload);

        Ok(self)
    }

    async fn sync(mut self, db_conn: &Database, id: Uuid) -> Result<Self> {
        let mut upload = self
            .uploads
            .remove(&id)
            .ok_or(Error::UploadNotFound)?
            .to_owned();

        let from = match upload.has_previous() {
            true => upload
                .previous_sha(db_conn)
                .await?
                .unwrap_or(self.git.empty_tree_sha()),
            false => self.git.empty_tree_sha(),
        };

        let diff = self.git.diff(&upload.dir(), &upload.sha, &from)?;

        sync(db_conn, &mut upload, diff).await?;

        self.uploads.insert(id, upload);

        Ok(self)
    }

    fn cleanup(self, id: Uuid) -> Result<Self> {
        let dir = self.uploads.get(&id).ok_or(Error::UploadNotFound)?.dir();
        std::fs::remove_dir_all(dir).map_err(|_| Error::CleanupFailure)?;
        Ok(self)
    }
}

// cof = cleanup on failure
// This can wrap an action that can fail and force it to clean up before continuing further.
fn cof<T>(dir: &str, r: Result<T>) -> Result<T> {
    match r {
        Ok(t) => Ok(t),
        Err(e) => {
            std::fs::remove_dir_all(dir).map_err(|_| Error::CleanupFailure)?;
            Err(e)
        }
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
        Git::new(&path).unwrap()
    }

    #[tokio::test]
    async fn test_upload() {
        let git = setup();

        let _uploader = Uploader::new(git);
    }
}

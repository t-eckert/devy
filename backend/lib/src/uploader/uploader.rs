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
    /// Create a new Uploader instance.
    pub fn new(git: Git) -> Self {
        Self {
            git,
            uploads: HashMap::new(),
        }
    }

    /// Upload a blog.
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

        let upload = upload.verify(db_conn).await?;
        if upload.status == "skipped" {
            self.uploads.insert(id, upload);
            return Ok(self);
        }

        let upload = upload.receive(db_conn).await?;
        let upload = cof(&upload.dir(), upload.clone_repo(db_conn, &self.git).await)?;

        self.uploads.insert(id, upload);

        Ok(self)
    }

    // sync updates the blog by applying the diff between the previous and current states.
    async fn sync(mut self, db_conn: &Database, id: Uuid) -> Result<Self> {
        let mut upload = self
            .uploads
            .remove(&id)
            .ok_or(Error::UploadNotFound)?
            .to_owned();

        if upload.status == "skipped" {
            self.uploads.insert(id, upload);
            return Ok(self);
        }

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

    // cleanup removes the upload directory from the filesystem.
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

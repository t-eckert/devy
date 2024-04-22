use super::{sync_posts, Diff};
use crate::{error::Result, Error, Git};
use db::{upload, Database};
use entities::{RepoMetadata, Upload};
use std::fs;
use uuid::Uuid;

/// The Uploader struct is responsible for handling the upload process.
#[derive(Clone)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    /// Create a new Uploader with the given Git instance.
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    /// Upload posts based on the upload object.
    pub async fn upload(self, db: &Database, mut upload: Upload) -> Result<Upload> {
        dbg!("uploading");

        upload::set_status(db, upload.id, "received").await?;
        upload::append_log(db, upload.id, "INFO: Upload received by uploader").await?;

        upload = Self::set_previous_upload(db, &upload).await?;

        // TODO cleanup when there are errors.
        self.clone_repo(db, upload.clone()).await?;
        upload = self.set_sha(db, upload).await?;
        self.sync_posts(db, upload.clone()).await?;
        Self::cleanup(upload.id)?;

        upload::set_status(db, upload.id, "done").await?;
        upload::append_log(db, upload.id, "INFO: Upload complete.").await?;

        Ok(upload::get_by_id(db, upload.id).await?)
    }

    pub async fn upload_new(self, db: &Database, upload: Upload) -> Result<Upload> {
        Self::receive(db, &upload).await?;
        self.clone_repo(db, upload.clone()).await?;
        let diff = self.diff(db, &upload).await?;
        Self::parse().await;
        Self::sync().await;
        Self::cleanup(upload.id)?;

        Ok(upload::get_by_id(db, upload.id).await?)
    }

    /// Receive sets the status of the upload to received, appends a log message,
    /// and sets the previous upload if it exists.
    async fn receive(db: &Database, upload: &Upload) -> Result<Upload> {
        upload::set_status(db, upload.id, "received").await?;
        upload::append_log(db, upload.id, "INFO: Upload received by uploader").await?;

        // Set the previous upload if it exists.
        Ok(match upload::get_previous(db, &upload.repo).await? {
            Some(previous) => {
                upload::set_previous(db, upload.id, previous.id).await?;
                upload::append_log(
                    db,
                    upload.id,
                    &format!("INFO: Previous upload {}", previous.id),
                )
                .await?
            }
            None => upload::append_log(db, upload.id, "INFO: No previous upload found").await?,
        })
    }

    /// Clone the repository.
    async fn clone_repo(&self, db: &Database, upload: Upload) -> Result<()> {
        println!(">>>>>cloning");
        self.git.clone_repo(&Self::dir(upload.id), &upload.repo)?;
        upload::set_status(db, upload.id, "cloned").await?;
        upload::append_log(db, upload.id, "INFO: Repository cloned").await?;
        Ok(())
    }

    async fn diff(&self, db: &Database, upload: &Upload) -> Result<Vec<Diff>> {
        Ok(vec![Diff::Added("".to_string())])
    }

    async fn parse() {}

    async fn sync() {}

    fn cleanup(id: Uuid) -> Result<()> {
        fs::remove_dir_all(&Self::dir(id)).map_err(|e| {
            Error::CleanupFailure(format!(
                "Failed to remove directory {}: {}",
                &Self::dir(id),
                e.to_string()
            ))
        })?;
        Ok(())
    }

    //--------------------------------------------------------------------------------

    /// Set the previous upload id for the upload if it exists.
    async fn set_previous_upload(db: &Database, upload: &Upload) -> Result<Upload> {
        Ok(match upload::get_previous(db, &upload.repo).await? {
            Some(previous) => {
                let id = upload.id;
                upload::set_previous(db, upload.id, previous.id).await?;
                upload::append_log(db, id, &format!("INFO: Previous upload {}", previous.id))
                    .await?
            }
            None => upload::append_log(db, upload.id, "INFO: No previous upload found").await?,
        })
    }

<<<<<<< HEAD
=======
    /// Clone the repository.
    async fn clone_repo(&self, db: &Database, upload: Upload) -> Result<()> {
        println!(">>>>>cloning");
        self.git.clone_repo(&Self::dir(upload.id), &upload.repo)?;
        upload::set_status(db, upload.id, "cloned").await?;
        upload::append_log(db, upload.id, "INFO: Repository cloned").await?;
        Ok(())
    }

>>>>>>> 301f6f9 (uploads: fix issue when receiving webhooks)
    /// Set the SHA for the upload.
    async fn set_sha(&self, db: &Database, upload: Upload) -> Result<Upload> {
        let sha = self.git.sha(&Self::dir(upload.id))?;
        upload::set_sha(db, upload.id, sha.clone()).await?;
        Ok(upload::append_log(db, upload.id, &format!("INFO: Current SHA {}", &sha)).await?)
    }

    async fn sync_posts(&self, db: &Database, upload: Upload) -> Result<()> {
        let from = upload::get_previous(db, &upload.repo)
            .await?
            .map(|u| u.sha)
            .unwrap_or(self.git.first_sha(&dir(upload.id))?);

        let diffs = self.git.diff(&dir(upload.id), &upload.sha, &from)?;
        sync_posts(db, upload.id, &dir(upload.id), diffs).await?;

        Ok(())
    }

    /// The directory where an upload will be cloned based on its ID.
    fn dir(id: Uuid) -> String {
        format!("/tmp/{}/", id)
    }

    /// Fetch the repository metadata from the GitHub API.
    async fn fetch_repo_metadata(api_url: &str) -> Result<RepoMetadata> {
        match reqwest::Client::new()
            .get(api_url)
            .header("User-Agent", "devy")
            .header("Accept", "application/json")
            .send()
            .await
        {
            Ok(response) => Ok(response
                .json()
                .await
                .map_err(|_| crate::Error::DependencyError("".to_string()))?),
            Err(err) => Err(crate::Error::DependencyError(err.to_string())),
        }
    }
}

/// The directory where an upload will be cloned based on its ID.
fn dir(id: Uuid) -> String {
    format!("/tmp/{}/", id)
}

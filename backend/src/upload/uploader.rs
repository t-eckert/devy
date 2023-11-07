use super::error::{Error, Result};
use super::git::Git;
use crate::entities::Upload;
use sqlx::PgPool;
use tracing::{event, Level};

#[derive(Clone)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    pub async fn upload(self, upload: Upload, pool: &PgPool) -> Result<Upload> {
        let received = upload
            .set_status(pool, "received".to_string())
            .await?
            .log(pool, "INFO: Upload received by uploader.".to_string())
            .await?;

        let url = received.clone().repo.ok_or(Error::RepositoryNotFound(
            "No repository present on received upload request.".to_string(),
        ))?;
        let dir = format!(
            "/tmp/{}",
            received.clone().id.ok_or(Error::RepositoryNotFound(
                "No id present on received upload request.".to_string(),
            ))?
        );
        self.git.clone_repo(&dir, &url)?;
        event!(Level::INFO, "Cloned repo {} to {}", url, dir);

        let cloned = received
            .set_status(pool, "cloned".to_string())
            .await?
            .log(pool, "INFO: Repository cloned.".to_string())
            .await?;

        Ok(cloned)
    }
}

use super::error::{Error, Result};
use super::git::Git;
use crate::entities::{Post, Upload};
use glob::glob;
use serde_with::NoneAsEmptyString;
use sqlx::PgPool;
use std::fs;
use std::path::PathBuf;
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

        let files = Self::get_markdown_files(&dir)?;
        for file in files {
            let post = Self::parse_file(file)?;
        }

        Ok(cloned)
    }

    pub async fn cleanup(self, upload: Upload, pool: &PgPool) -> Result<Upload> {
        let cleaning = upload
            .set_status(pool, "cleaning".to_string())
            .await?
            .log(pool, "INFO: Cleaning up repository.".to_string())
            .await?;

        let dir = format!(
            "/tmp/{}",
            cleaning.clone().id.ok_or(Error::RepositoryNotFound(
                "No id present on received upload request.".to_string(),
            ))?
        );

        fs::remove_dir_all(&dir).map_err(|e| {
            Error::CleanupFailure(format!(
                "Failed to remove directory {}: {}",
                &dir,
                e.to_string()
            ))
        })?;

        let done = cleaning
            .set_status(pool, "done".to_string())
            .await?
            .log(pool, "INFO: Upload complete.".to_string())
            .await?;

        Ok(done)
    }

    fn get_markdown_files(dir: &str) -> Result<Vec<PathBuf>> {
        Ok(glob(format!("{}/**/*.md", dir).as_str())
            .map_err(|e| Error::FileParseError(e.to_string()))?
            .filter_map(|x| x.ok())
            .filter(|x| x.is_file())
            // TODO this is probably slow...
            .filter(|x| !format!("{}", x.display()).contains("README"))
            .collect::<Vec<PathBuf>>())
    }

    fn parse_file(filename: PathBuf) -> Result<Post> {
        let slug = filename
            .file_stem()
            .ok_or(Error::FileParseError(
                "Failed to get file stem.".to_string(),
            ))?
            .to_str()
            .ok_or(Error::FileParseError(
                "Failed to convert file stem to string.".to_string(),
            ))?
            .to_string();

        let contents =
            fs::read_to_string(filename).map_err(|e| Error::FileParseError(e.to_string()))?;

        Ok(Post::new(slug, contents))
    }
}

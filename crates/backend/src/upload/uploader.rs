use glob::glob;
use regex::Regex;
use sqlx::PgPool;
use std::fs;
use std::path::PathBuf;
use tracing::{event, Level};

use super::{
    error::{Error, Result},
    git::Git,
};
use crate::entities::{blog, post, repo, upload, Upload};

#[derive(Clone)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    pub async fn upload(self, upload: Upload, pool: &PgPool) -> Result<Upload> {
        upload::set_status(pool, upload.id, "received").await?;
        upload::append_log(pool, upload.id, "INFO: Upload received by uploader.").await?;

        let url = upload.repo.clone();
        let repo = repo::get_by_url(pool, &upload.repo).await?;
        let blog = blog::get_by_id(pool, repo.blog_id).await?;

        let dir = format!("/tmp/{}", upload.id);
        self.git.clone_repo(&dir, &url)?;
        event!(Level::INFO, "Cloned repo {} to {}", url, dir);

        upload::set_status(pool, upload.id, "cloned").await?;
        upload::append_log(pool, upload.id, "INFO: Repository cloned.").await?;

        let files = Self::get_markdown_files(&dir)?;
        let mut posts = vec![];
        for file in files {
            let post = Self::parse_file(file)?
                .title_from_content()
                .blog_slug(blog.slug.clone());

            posts.push(post);
        }

        for post in posts {
            match post::upsert(pool, post).await {
                Ok(post) => {
                    upload::append_log(pool, upload.id, &format!("INFO: Uploaded {}", post.title))
                        .await?;
                }
                Err(_) => {}
            };
        }

        upload::set_status(pool, upload.id, "uploaded").await?;
        upload::append_log(pool, upload.id, "INFO: Cleaning up repository.").await?;

        fs::remove_dir_all(&dir).map_err(|e| {
            Error::CleanupFailure(format!(
                "Failed to remove directory {}: {}",
                &dir,
                e.to_string()
            ))
        })?;

        upload::set_status(pool, upload.id, "done").await?;
        upload::append_log(pool, upload.id, "INFO: Upload complete.").await?;

        Ok(upload::get_by_id(pool, upload.id).await?)
    }

    fn get_markdown_files(dir: &str) -> Result<Vec<PathBuf>> {
        Ok(glob(format!("{}/**/*.md", dir).as_str())
            .map_err(|e| Error::FileParseError(e.to_string()))?
            .filter_map(|x| x.ok())
            .filter(|x| x.is_file())
            .filter(|x| !format!("{}", x.display()).contains("README"))
            .collect::<Vec<PathBuf>>())
    }

    fn parse_file(filename: PathBuf) -> Result<post::PostForUpsert> {
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

        Ok(post::PostForUpsert::new(slug, contents))
    }

    fn markdown_title(markdown: &str) -> Option<String> {
        let title_pattern = Regex::new(r"^#\s+(.+)").unwrap();

        for line in markdown.lines() {
            // Attempt to match the line against the title pattern
            if let Some(captures) = title_pattern.captures(line) {
                // Extract the captured title group and return it
                if let Some(title) = captures.get(1) {
                    return Some(title.as_str().to_string());
                }
            }
        }

        None
    }
}
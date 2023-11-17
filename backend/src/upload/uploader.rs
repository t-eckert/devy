use super::{
    error::{Error, Result},
    git::Git,
};
use crate::entities::{Blog, Post, Profile, Repo, Upload, User};
use glob::glob;
use regex::Regex;
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

        let repo = Repo::get_by_url(pool, &url).await?;
        let blog = Blog::get_by_id(
            pool,
            repo.blog_id.ok_or(Error::DependencyError(
                "Repo does not have a blog associated with it".to_string(),
            ))?,
        )
        .await?;
        let profile = Profile::get_by_username(pool, blog.username.clone()).await?;
        let user = User::get_by_username(pool, blog.username.clone()).await?;

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
        let mut posts = vec![];
        for file in files {
            let mut post = Self::parse_file(file)?;
            post.title = Self::markdown_title(&post.content.clone().ok_or(
                Error::DependencyError("Post does not have content.".to_string()),
            )?);
            post.blog_slug = Some(blog.slug.clone());
            post.blog_name = Some(blog.name.clone());
            post.author_name = profile.display_name.clone();
            post.author_slug = Some(user.username.clone());

            posts.push(post);
        }

        for post in posts {
            match post.insert(pool).await {
                Ok(post) => {
                    cloned
                        .clone()
                        .log(
                            pool,
                            format!(
                                "INFO: Uploaded {}",
                                post.title.unwrap_or("Untitled".to_string())
                            ),
                        )
                        .await?;
                }
                Err(_) => {}
            };
        }

        // TODO I need to set this up so it _ALWAYS_ runs cleanup even when there's an error.
        let cleaning = cloned
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

use crate::db::{blog, profile, repo, user, Database};
use crate::entities::{Blog, Repo};
use crate::forms::error::{Error, Result};
use serde::{Deserialize, Serialize};
use slugify::slugify;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Represents a form submission for creating a new blog.
pub struct NewBlog {
    /// The user_id of the user creating the blog.
    pub user_id: Uuid,
    /// The Devy username of the user creating the blog.
    pub username: String,
    /// The name of the blog.z
    pub name: String,
    /// The URL where the repo can be cloned from.
    pub repo_url: String,
    /// The slug of the blog.
    pub slug: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NewBlogResponse {
    blog: Blog,
    repo: Repo,
}

impl NewBlog {
    // Process takes the form submission, validates it, then creates the necessary entities
    // defined in the form.
    // It will create a blog entity and a repo entity and return both as a Response.
    pub async fn process(self, db: &Database) -> Result<NewBlogResponse> {
        self.validate(db).await?;

        let profile = profile::get_by_username(db, self.username.clone()).await?;
        let blog = blog::upsert(db, profile.id, &self.name, &self.slug, None).await?;
        let repo = repo::upsert(db, blog.id, self.repo_url).await?;

        Ok(NewBlogResponse { blog, repo })
    }

    /// Validate the form.
    pub async fn validate(&self, db: &Database) -> Result<()> {
        self.validate_repo_url()?;
        self.validate_user_exists(db).await?;
        self.validate_slug_not_taken(db).await?;
        Ok(())
    }

    fn validate_repo_url(&self) -> Result<()> {
        let url = match url::Url::parse(&self.repo_url) {
            Ok(url) => url,
            Err(_) => {
                return Err(Error::Malformed {
                    message: "Invalid URL".to_string(),
                })
            }
        };
        if url.scheme() != "https" {
            return Err(Error::Malformed {
                message: "Invalid URL".to_string(),
            });
        }
        if url.host_str() != Some("github.com") {
            return Err(Error::Malformed {
                message: "Invalid URL".to_string(),
            });
        }

        Ok(())
    }

    async fn validate_user_exists(&self, db: &Database) -> Result<()> {
        user::get_by_username(db, &self.username).await?;
        Ok(())
    }

    async fn validate_slug_not_taken(&self, db: &Database) -> Result<()> {
        match blog::get_by_slug(db, &self.slug).await {
            Ok(blog) => {Err(Error::Conflict {
                message: "Slug already taken".to_string(),
            })
            },
            Err(err) => {
                if let crate::db::Error::EntityNotFound = err {
                    return Ok(());
                }

                Err(err.into())
            }
        }
    }
}

fn slug(s: &str) -> String {
    slugify!(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_validate_with_nonexistent_user(db: Database) {
        let new_blog = NewBlog {
            user_id: Uuid::new_v4(),
            username: "nonexistennnntuser".to_string(),
            name: "No thing...".to_string(),
            repo_url: "https://github.com/t-eckert/devy".to_string(),
            slug: "no-thing".to_string(),
            description: None,
        };

        let actual = new_blog.validate(&db).await;

        assert!(actual.is_err());
    }
}

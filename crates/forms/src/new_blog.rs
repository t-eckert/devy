use crate::error::{Error, Result};
use db::{blog, profile, repo, user, Database};
use entities::{Blog, Repo};
use serde::{Deserialize, Serialize};
use slugify::slugify;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Represents a form submission for creating a new blog.
pub struct NewBlog {
    /// The Devy username of the user creating the blog.
    pub username: String,
    /// The name of the blog.
    pub name: String,
    /// The URL where metadata about the repo can be retrieved.
    pub repo_url: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NewBlogResponse {
    blog: Blog,
    repo: Repo,
}

impl NewBlog {
    pub fn new(username: String, name: String, repo_url: String) -> Self {
        Self {
            username,
            name,
            repo_url,
        }
    }

    // Process takes the form submission, validates it, then creates the necessary entities
    // defined in the form.
    // It will create a blog entity and a repo entity and return both as a Response.
    pub async fn process(self, db: &Database) -> Result<NewBlogResponse> {
        self.validate(db).await?;

        let profile = profile::get_by_username(&db, self.username.clone()).await?;
        let blog = blog::upsert(&db, profile.id, &self.name, &slug(&self.name), None).await?;
        let repo = repo::upsert(&db, blog.id, self.repo_url).await?;

        // TODO Abstract this somewhere... I'm just not sure where yet.

        Ok(NewBlogResponse { blog, repo })
    }

    pub async fn validate(&self, db: &Database) -> Result<()> {
        self.validate_repo_url()?;
        self.validate_user_exists(&db).await?;
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
        if url.host_str() != Some("api.github.com") {
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
}

fn slug(s: &str) -> String {
    slugify!(s)
}

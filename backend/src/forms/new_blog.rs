use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::*;
use crate::entities::{blog, profile, repo, user, Blog, Profile, Repo};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
/// Represents a form submission for creating a new blog.
pub struct NewBlog {
    pub username: String,
    pub name: String,
    pub repo_url: String,
    pub github_id: i64,
    pub github_name: String,
}

pub struct Response {
    blog: Blog,
    repo: Repo,
}

impl NewBlog {
    pub fn new(
        username: String,
        name: String,
        repo_url: String,
        github_id: i64,
        github_username: String,
    ) -> Self {
        Self {
            username,
            name,
            repo_url,
            github_id,
            github_name: github_username,
        }
    }

    /// Process takes the form submission, validates it, then creates the necessary entities
    /// defined in the form.
    /// It will create a blog entity and a repo entity and return both as a Response.
    pub async fn process(self, pool: &PgPool) -> Result<Response> {
        validate_repo_url(&self.repo_url)?;
        self.validate_user_exists(pool).await?;

        let profile = profile::get_by_username(pool, self.username.clone()).await?;
        let blog = self.create_blog(pool, profile).await?;
        let repo = self.create_repo(pool, &blog.id).await?;

        Ok(Response { blog, repo })
    }

    async fn validate_user_exists(&self, pool: &PgPool) -> Result<()> {
        user::get_by_username(pool, &self.username).await?;
        Ok(())
    }

    async fn create_blog(&self, pool: &PgPool, profile: Profile) -> Result<Blog> {
        let blog = blog::BlogForUpsert::new(
            self.name.clone(),
            blog_slug_from_repo(&self.repo_url)?,
            Uuid::parse_str(&profile.id.unwrap()).unwrap(), // TODO Remove this when the profile is using
                                                            // UUID
        );
        Ok(blog::upsert(pool, blog).await?)
    }

    async fn create_repo(&self, pool: &PgPool, blog_id: &Uuid) -> Result<Repo> {
        let repo = repo::RepoForUpsert::new(
            blog_id.clone(),
            self.repo_url.clone(),
            self.github_id.clone(),
            self.github_name.clone(),
        );
        Ok(repo::upsert(pool, repo).await?)
    }
}

fn validate_repo_url(repo_url: &str) -> Result<()> {
    let url = match url::Url::parse(&repo_url) {
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

fn blog_slug_from_repo(repo_url: &str) -> Result<String> {
    let url = match url::Url::parse(repo_url) {
        Ok(url) => url,
        Err(_) => {
            return Err(Error::Malformed {
                message: "Invalid URL".to_string(),
            })
        }
    };
    let path = url.path();
    let path_parts: Vec<&str> = path.split('/').collect();
    let repo_name = path_parts.get(2).ok_or(Error::Malformed {
        message: "Invalid Url".to_string(),
    })?;
    Ok(repo_name.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_repo_url_valid() {
        let url = "https://github.com/username/repo";
        assert!(validate_repo_url(url).is_ok());
    }

    #[test]
    fn test_validate_repo_url_invalid_unparsable() {
        let url = "pete's pizza";
        assert!(validate_repo_url(url).is_err());
    }

    #[test]
    fn test_validate_repo_url_invalid_scheme() {
        let url = "http://github.com/username/repo";
        assert!(validate_repo_url(url).is_err());
    }

    #[test]
    fn test_validate_repo_url_invalid_host() {
        let url = "https://goathub.com/username/repo";
        assert!(validate_repo_url(url).is_err());
    }

    #[test]
    fn test_blog_slug_from_repo_valid() {
        let url = "https://github.com/username/repo";
        assert_eq!(blog_slug_from_repo(url).unwrap(), "repo");
    }
}

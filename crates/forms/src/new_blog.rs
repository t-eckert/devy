use crate::error::Result;
use db::{blog, profile, repo, user, Database};
use entities::{Blog, Profile, Repo};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Represents a form submission for creating a new blog.
pub struct NewBlog {
    /// The Devy username of the user creating the blog.
    pub username: String,
    /// The name of the blog.
    pub name: String,
    /// The URL of the blog's repository.
    /// Must be a valid GitHub repository URL.
    pub repo_url: String,
    /// The GitHub ID of the user creating the blog.
    pub github_id: i64,
    /// The GitHub username of the user creating the blog.
    pub github_name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
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
        github_name: String,
    ) -> Self {
        Self {
            username,
            name,
            repo_url,
            github_id,
            github_name,
        }
    }

    // Process takes the form submission, validates it, then creates the necessary entities
    // defined in the form.
    // It will create a blog entity and a repo entity and return both as a Response.
    // pub async fn process(self, db: &Database) -> Result<Response> {
    // validate_repo_url(&self.repo_url)?;
    // self.validate_user_exists(pool).await?;
    //
    // let profile = profile::get_by_username(pool, self.username.clone()).await?;
    // let blog = self.create_blog(pool, profile).await?;
    // let repo = self.create_repo(pool, &blog.id).await?;
    //
    // Ok(Response { blog, repo })
    // }

    // async fn validate_user_exists(&self, db: &Database) -> Result<()> {
    //     user::get_by_username(db, &self.username).await?;
    //     Ok(())
    // }
    //
    // async fn create_blog(&self, db: &Database, profile: Profile) -> Result<Blog> {
    //     Ok(blog::upsert(
    //         db,
    //         &self.name,
    //         blog_slug_from_repo(&self.repo_url)?,
    //         Uuid::parse_str(&profile.id.unwrap()).unwrap(), // TODO Remove this when the profile is using
    //     )
    //     .await?)
    // }
    //
    // async fn create_repo(&self, pool: &Database, blog_id: &Uuid) -> Result<Repo> {
    //     let repo = repo::RepoForUpsert::new(
    //         blog_id.clone(),
    //         self.repo_url.clone(),
    //         self.github_id.clone(),
    //         self.github_name.clone(),
    //     );
    //     Ok(repo::upsert(pool, repo).await?)
    // }
}

// fn validate_repo_url(repo_url: &str) -> Result<()> {
//     let url = match url::Url::parse(&repo_url) {
//         Ok(url) => url,
//         Err(_) => {
//             return Err(Error::Malformed {
//                 message: "Invalid URL".to_string(),
//             })
//         }
//     };
//     if url.scheme() != "https" {
//         return Err(Error::Malformed {
//             message: "Invalid URL".to_string(),
//         });
//     }
//     if url.host_str() != Some("github.com") {
//         return Err(Error::Malformed {
//             message: "Invalid URL".to_string(),
//         });
//     }
//
//     Ok(())
// }
//
// fn blog_slug_from_repo(repo_url: &str) -> Result<String> {
//     let url = match url::Url::parse(repo_url) {
//         Ok(url) => url,
//         Err(_) => {
//             return Err(Error::Malformed {
//                 message: "Invalid URL".to_string(),
//             })
//         }
//     };
//     let path = url.path();
//     let path_parts: Vec<&str> = path.split('/').collect();
//     let repo_name = path_parts.get(2).ok_or(Error::Malformed {
//         message: "Invalid Url".to_string(),
//     })?;
//     Ok(repo_name.to_string())
// }
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_validate_repo_url_valid() {
//         let url = "https://github.com/username/repo";
//         assert!(validate_repo_url(url).is_ok());
//     }
//
//     #[test]
//     fn test_validate_repo_url_invalid_unparsable() {
//         let url = "pete's pizza";
//         assert!(validate_repo_url(url).is_err());
//     }
//
//     #[test]
//     fn test_validate_repo_url_invalid_scheme() {
//         let url = "http://github.com/username/repo";
//         assert!(validate_repo_url(url).is_err());
//     }
//
//     #[test]
//     fn test_validate_repo_url_invalid_host() {
//         let url = "https://goathub.com/username/repo";
//         assert!(validate_repo_url(url).is_err());
//     }
//
//     #[test]
//     fn test_blog_slug_from_repo_valid() {
//         let url = "https://github.com/username/repo";
//         assert_eq!(blog_slug_from_repo(url).unwrap(), "repo");
//     }
// }

use crate::{date::Date, profile::Profile, user::User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Blog represents a user's blog and points to the user's profile and user id.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    /// The unique identifier for the blog.
    pub id: Uuid,
    /// The unique identifier for the profile that owns the blog.
    pub profile_id: Uuid,
    /// The unique identifier for the user that owns the blog.
    pub user_id: Uuid,

    /// The username of the author of the blog.
    pub author_username: String,
    /// The display name of the author of the blog.
    pub author_display_name: Option<String>,

    /// The name of the blog.
    pub name: String,
    /// The slug of the blog. Must be unique.
    pub slug: String,
    /// The description of the blog.
    pub description: Option<String>,

    /// The Git repository URL for the blog.
    pub repo_url: String,

    /// When the blog was first created.
    pub created_at: Date,
    /// When the blog was last updated.
    pub updated_at: Date,
}

impl Blog {
    pub fn new(profile: &Profile, user: &User, name: &str, slug: &str, repo_url: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            profile_id: profile.id,
            user_id: user.id,
            author_username: user.username.clone(),
            author_display_name: profile.display_name.clone(),
            name: name.to_string(),
            slug: slug.to_string(),
            description: None,
            repo_url: repo_url.to_string(),
            created_at: Date::now(),
            updated_at: Date::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_blog() {
        let user = User::new("alice", Some("alice@email.com"));
        let profile = Profile::new(user.id, "Alice");

        let blog = Blog::new(
            &profile,
            &user,
            "Alice's Blog",
            "alices-blog",
            "https://github.com/alice/alices-blog",
        );

        assert_eq!(blog.author_username, "alice");
    }
}

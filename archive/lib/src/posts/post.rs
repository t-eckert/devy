use crate::{blogs::Blog, date::Date};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Post represents a blog post.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    /// The unique identifier for the post.
    pub id: Uuid,
    /// The slug of the post.
    pub slug: String,

    /// The unique identifier of the blog that the post belongs to.
    pub blog_id: Uuid,
    /// The slug of the blog that the post belongs to.
    pub blog_slug: String,
    /// The name of the blog that the post belongs to.
    pub blog_name: Option<String>,

    /// The slug of the profile of the author of the post.
    pub author_slug: Option<String>,
    /// The name of the author of the post.
    pub author_name: Option<String>,

    // The date the post was created.
    pub created_at: Date,
    // The date the post was last updated.
    pub updated_at: Date,

    /// The title of the post.
    pub title: String,
    /// The number of likes the post has.
    pub like_count: Option<i32>,
    /// Whether the post is a draft.
    pub is_draft: bool,

    /// The body of the post.
    pub body: String,
}

impl Post {
    /// Create a new post.
    pub fn new(blog: &Blog, slug: &str, title: &str, body: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            blog_id: blog.id,
            slug: slug.to_string(),
            blog_slug: blog.slug.to_string(),
            blog_name: Some(blog.name.to_string()),
            author_slug: Some(blog.author_username.to_string()),
            author_name: blog.author_display_name.as_deref().map(|s| s.to_string()),
            created_at: Date::now(),
            updated_at: Date::now(),
            title: title.to_string(),
            like_count: Some(0),
            is_draft: false,
            body: body.to_string(),
        }
    }
}

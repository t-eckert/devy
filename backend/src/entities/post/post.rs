use super::*;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::{uuid, Uuid};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Post {
    pub id: Uuid,
    pub slug: String,

    pub blog_slug: Option<String>,
    pub blog_name: Option<String>,
    pub author_name: Option<String>,
    pub author_username: Option<String>,

    pub title: String,
    pub content: String,

    pub likes: Option<i64>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostForUpsert {
    pub id: Option<Uuid>,
    pub slug: Option<String>,

    pub blog_slug: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
}

impl PostForUpsert {
    pub fn new(slug: String, title: String) -> Self {
        Self {
            id: None,
            slug: Some(slug),
            blog_slug: None,
            title: Some(title),
            content: None,
        }
    }

    pub fn blog_slug(mut self, blog_slug: String) -> Self {
        self.blog_slug = Some(blog_slug);
        self
    }

    pub fn title_from_content(mut self) -> Self {
        let title = self
            .content
            .clone()
            .ok_or(Error::MissingField("content".to_string()))
            .and_then(|content| Self::markdown_title(&content))
            .unwrap_or("Untitled".to_string());

        self.title = Some(title);
        self
    }

    fn markdown_title(markdown: &str) -> Result<String> {
        let title_pattern = Regex::new(r"^#\s+(.+)").unwrap();

        for line in markdown.lines() {
            // Attempt to match the line against the title pattern
            if let Some(captures) = title_pattern.captures(line) {
                // Extract the captured title group and return it
                if let Some(title) = captures.get(1) {
                    return Ok(title.as_str().to_string());
                }
            }
        }

        Err(Error::MissingField("title".to_string()))
    }
}

use super::changeset::{
    Changeset, PostAddition, PostChange, PostChangeset, PostDeletion, PostModification,
};
use uuid::Uuid;

pub struct ChangesetBuilder {
    blog_id: Option<Uuid>,
    blog_slug: Option<String>,
    dir: Option<String>,
    diff: Option<String>,
}

impl ChangesetBuilder {
    pub fn new() -> Self {
        ChangesetBuilder {
            blog_id: None,
            blog_slug: None,
            dir: None,
            diff: None,
        }
    }

    pub fn with_blog_id(mut self, blog_id: Uuid) -> Self {
        self.blog_id = Some(blog_id);
        self
    }

    pub fn with_blog_slug(mut self, blog_slug: &str) -> Self {
        self.blog_slug = Some(blog_slug.to_string());
        self
    }

    pub fn with_dir(mut self, dir: &str) -> Self {
        self.dir = Some(dir.to_string());
        self
    }

    pub fn with_diff(mut self, diff: &str) -> Self {
        self.diff = Some(diff.to_string());
        self
    }

    pub fn build(&self) -> Result<Changeset, anyhow::Error> {
        let blog_id = self
            .blog_id
            .clone()
            .ok_or(anyhow::anyhow!("Blog ID not provided"))?;
        let blog_slug = self
            .blog_slug
            .clone()
            .ok_or(anyhow::anyhow!("Blog slug not provided"))?;
        let dir = self
            .dir
            .clone()
            .ok_or(anyhow::anyhow!("Dir not provided"))?;
        let diff = self
            .diff
            .clone()
            .ok_or(anyhow::anyhow!("Diff not provided"))?;

        let mut changeset = Changeset {
            blog_id,
            blog_slug,
            dir,
            post_changeset: PostChangeset {
                post_changes: vec![],
            },
        };

        for line in diff.lines() {
            match line.chars().next() {
                Some('A') => {
                    let path = line.split_whitespace().skip(1).collect();
                    changeset
                        .post_changeset
                        .post_changes
                        .push(PostChange::Addition(PostAddition { path }));
                }
                Some('D') => {
                    let path = line.split_whitespace().skip(1).collect();
                    changeset
                        .post_changeset
                        .post_changes
                        .push(PostChange::Deletion(PostDeletion { path }));
                }
                Some('M') => {
                    let path = line.split_whitespace().skip(1).collect();
                    changeset
                        .post_changeset
                        .post_changes
                        .push(PostChange::Modification(PostModification {
                            path,
                            from_path: None,
                        }));
                }
                Some('R') => {
                    let mut parts = line.split_whitespace().skip(1);
                    let path = parts.next().unwrap();
                    let from_path = parts.next().unwrap();
                    changeset
                        .post_changeset
                        .post_changes
                        .push(PostChange::Modification(PostModification {
                            path: path.to_string(),
                            from_path: Some(from_path.to_string()),
                        }));
                }
                _ => {}
            }
        }

        Ok(changeset)
    }
}

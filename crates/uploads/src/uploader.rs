use crate::{
    diff::{diff_files, Diff},
    error::{Error, Result},
    Git,
};
use db::{post, repo, upload, Database};
use entities::{Post, Repo, Upload};
use glob::glob;
use serde_json::json;
use std::fs;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Clone)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    pub async fn upload(self, db: &Database, mut upload: Upload) -> Result<Upload> {
        // Set the status of the upload to "received" and append a log message.
        upload = upload::set_status(db, upload.id, "received").await?;
        upload = upload::append_log(db, upload.id, "INFO: Upload received by uploader").await?;

        // Fetch the repository metadata from the GitHub API.
        let repo_metadata = Self::fetch_repo_metadata(upload.repo.clone()).await?;

        // Check for the previous upload.
        let previous_upload = upload::get_previous(db, &upload.repo).await?;
        upload.previous_upload_id = previous_upload.clone().map(|u| u.id);
        match upload.previous_upload_id {
            Some(previous) => {
                upload::set_previous(db, upload.id, previous).await?;
                upload::append_log(
                    db,
                    upload.id,
                    &format!("INFO: Previous upload found: {}", previous),
                )
                .await?;
            }
            None => {
                upload::append_log(db, upload.id, "INFO: No previous upload found").await?;
            }
        }

        // Clone the repository.
        let dir = format!("/tmp/{}", upload.id);
        self.git.clone_repo(&dir, &repo_metadata.clone_url)?;
        upload::set_status(db, upload.id, "cloned").await?;
        upload::append_log(db, upload.id, "INFO: Repository cloned").await?;

        // Set the SHA for the upload.
        let sha = self.git.sha(&dir)?;
        upload = upload::set_sha(db, upload.id, sha.clone()).await?;
        upload::append_log(db, upload.id, &format!("INFO: Current SHA {}", &sha)).await?;

        // Diff posts.
        let diff = match previous_upload {
            Some(previous) => diff_files(self.git.diff(&dir, &sha, &previous.sha)?),
            None => diff_files(self.git.diff(&dir, &sha, &self.git.first_sha(&dir)?)?),
        };
        dbg!(&diff);

        // Sync posts.
        let blog_slug = repo_metadata.name;
        Self::sync_posts(db, blog_slug, diff).await?;

        // Delete the temporary directory.
        Self::cleanup(&dir)?;
        upload::set_status(db, upload.id, "done").await?;
        upload::append_log(db, upload.id, "INFO: Upload complete.").await?;

        Ok(upload::get_by_id(db, upload.id).await?)
    }

    async fn fetch_repo_metadata(api_url: String) -> Result<RepoMetadata> {
        match reqwest::Client::new()
            .get(&api_url)
            .header("User-Agent", "devy")
            .header("Accept", "application/json")
            .send()
            .await
        {
            Ok(response) => Ok(response
                .json()
                .await
                .map_err(|_| crate::Error::DependencyError("".to_string()))?),
            Err(err) => Err(crate::Error::DependencyError(err.to_string())),
        }
    }

    async fn sync_posts(db: &Database, blog_slug: String, diffs: Vec<Diff>) -> Result<()> {
        for diff in diffs {
            dbg!(&diff);
            match diff {
                Diff::Added(file) => {
                    if file_is_markdown(&file) {
                        Self::add_post(db, &blog_slug.clone(), file).await?;
                    }
                }
                Diff::Modified(file) => {
                    if file_is_markdown(&file) {
                        Self::modify_post(db, file).await?;
                    }
                }
                Diff::Renamed(_, from, to) => {
                    if file_is_markdown(&to) {
                        Self::rename_post(db, from, to).await?;
                    }
                }
                Diff::Deleted(file) => {
                    if file_is_markdown(&file) {
                        Self::delete_post(db, file).await?;
                    }
                }
            }
        }

        Ok(())
    }

    async fn add_post(db: &Database, blog_slug: &str, path: String) -> Result<()> {
        println!("Adding post: {}", path);
        Ok(())
    }

    async fn modify_post(db: &Database, path: String) -> Result<()> {
        println!("Modifying post: {}", path);
        Ok(())
    }

    async fn rename_post(db: &Database, from: String, to: String) -> Result<()> {
        println!("Renaming post: {} -> {}", from, to);
        Ok(())
    }

    async fn delete_post(db: &Database, path: String) -> Result<()> {
        println!("Deleting post: {}", path);
        Ok(())
    }

    fn get_markdown_files(dir: &str) -> Result<Vec<MarkdownFile>> {
        let paths = glob(format!("{}/**/*.md", dir).as_str())
            .map_err(|e| Error::FileParseError(e.to_string()))?
            .filter_map(|x| x.ok())
            .filter(|x| x.is_file())
            .filter(|x| !format!("{}", x.display()).contains("README"))
            .collect::<Vec<PathBuf>>();

        let mut files = vec![];
        for path in paths {
            let content =
                fs::read_to_string(&path).map_err(|e| Error::FileParseError(e.to_string()))?;
            files.push(MarkdownFile {
                path: path.to_str().unwrap().to_string(),
                content,
            });
        }

        Ok(files)
    }

    fn cleanup(dir: &str) -> Result<()> {
        fs::remove_dir_all(&dir).map_err(|e| {
            Error::CleanupFailure(format!(
                "Failed to remove directory {}: {}",
                &dir,
                e.to_string()
            ))
        })?;
        Ok(())
    }
}

fn file_is_markdown(file: &str) -> bool {
    file.ends_with(".md")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MarkdownFile {
    path: String,
    content: String,
}

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoMetadata {
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    pub name: String,
    #[serde(rename = "full_name")]
    pub full_name: String,
    pub private: bool,
    pub owner: Owner,
    #[serde(rename = "html_url")]
    pub html_url: String,
    pub description: Value,
    pub fork: bool,
    pub url: String,
    #[serde(rename = "forks_url")]
    pub forks_url: String,
    #[serde(rename = "keys_url")]
    pub keys_url: String,
    #[serde(rename = "collaborators_url")]
    pub collaborators_url: String,
    #[serde(rename = "teams_url")]
    pub teams_url: String,
    #[serde(rename = "hooks_url")]
    pub hooks_url: String,
    #[serde(rename = "issue_events_url")]
    pub issue_events_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "assignees_url")]
    pub assignees_url: String,
    #[serde(rename = "branches_url")]
    pub branches_url: String,
    #[serde(rename = "tags_url")]
    pub tags_url: String,
    #[serde(rename = "blobs_url")]
    pub blobs_url: String,
    #[serde(rename = "git_tags_url")]
    pub git_tags_url: String,
    #[serde(rename = "git_refs_url")]
    pub git_refs_url: String,
    #[serde(rename = "trees_url")]
    pub trees_url: String,
    #[serde(rename = "statuses_url")]
    pub statuses_url: String,
    #[serde(rename = "languages_url")]
    pub languages_url: String,
    #[serde(rename = "stargazers_url")]
    pub stargazers_url: String,
    #[serde(rename = "contributors_url")]
    pub contributors_url: String,
    #[serde(rename = "subscribers_url")]
    pub subscribers_url: String,
    #[serde(rename = "subscription_url")]
    pub subscription_url: String,
    #[serde(rename = "commits_url")]
    pub commits_url: String,
    #[serde(rename = "git_commits_url")]
    pub git_commits_url: String,
    #[serde(rename = "comments_url")]
    pub comments_url: String,
    #[serde(rename = "issue_comment_url")]
    pub issue_comment_url: String,
    #[serde(rename = "contents_url")]
    pub contents_url: String,
    #[serde(rename = "compare_url")]
    pub compare_url: String,
    #[serde(rename = "merges_url")]
    pub merges_url: String,
    #[serde(rename = "archive_url")]
    pub archive_url: String,
    #[serde(rename = "downloads_url")]
    pub downloads_url: String,
    #[serde(rename = "issues_url")]
    pub issues_url: String,
    #[serde(rename = "pulls_url")]
    pub pulls_url: String,
    #[serde(rename = "milestones_url")]
    pub milestones_url: String,
    #[serde(rename = "notifications_url")]
    pub notifications_url: String,
    #[serde(rename = "labels_url")]
    pub labels_url: String,
    #[serde(rename = "releases_url")]
    pub releases_url: String,
    #[serde(rename = "deployments_url")]
    pub deployments_url: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "pushed_at")]
    pub pushed_at: String,
    #[serde(rename = "git_url")]
    pub git_url: String,
    #[serde(rename = "ssh_url")]
    pub ssh_url: String,
    #[serde(rename = "clone_url")]
    pub clone_url: String,
    #[serde(rename = "svn_url")]
    pub svn_url: String,
    pub homepage: Value,
    pub size: i64,
    #[serde(rename = "stargazers_count")]
    pub stargazers_count: i64,
    #[serde(rename = "watchers_count")]
    pub watchers_count: i64,
    pub language: Value,
    #[serde(rename = "has_issues")]
    pub has_issues: bool,
    #[serde(rename = "has_projects")]
    pub has_projects: bool,
    #[serde(rename = "has_downloads")]
    pub has_downloads: bool,
    #[serde(rename = "has_wiki")]
    pub has_wiki: bool,
    #[serde(rename = "has_pages")]
    pub has_pages: bool,
    #[serde(rename = "has_discussions")]
    pub has_discussions: bool,
    #[serde(rename = "forks_count")]
    pub forks_count: i64,
    #[serde(rename = "mirror_url")]
    pub mirror_url: Value,
    pub archived: bool,
    pub disabled: bool,
    #[serde(rename = "open_issues_count")]
    pub open_issues_count: i64,
    pub license: Value,
    #[serde(rename = "allow_forking")]
    pub allow_forking: bool,
    #[serde(rename = "is_template")]
    pub is_template: bool,
    #[serde(rename = "web_commit_signoff_required")]
    pub web_commit_signoff_required: bool,
    pub topics: Vec<Value>,
    pub visibility: String,
    pub forks: i64,
    #[serde(rename = "open_issues")]
    pub open_issues: i64,
    pub watchers: i64,
    #[serde(rename = "default_branch")]
    pub default_branch: String,
    #[serde(rename = "temp_clone_token")]
    pub temp_clone_token: Value,
    #[serde(rename = "network_count")]
    pub network_count: i64,
    #[serde(rename = "subscribers_count")]
    pub subscribers_count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Owner {
    pub login: String,
    pub id: i64,
    #[serde(rename = "node_id")]
    pub node_id: String,
    #[serde(rename = "avatar_url")]
    pub avatar_url: String,
    #[serde(rename = "gravatar_id")]
    pub gravatar_id: String,
    pub url: String,
    #[serde(rename = "html_url")]
    pub html_url: String,
    #[serde(rename = "followers_url")]
    pub followers_url: String,
    #[serde(rename = "following_url")]
    pub following_url: String,
    #[serde(rename = "gists_url")]
    pub gists_url: String,
    #[serde(rename = "starred_url")]
    pub starred_url: String,
    #[serde(rename = "subscriptions_url")]
    pub subscriptions_url: String,
    #[serde(rename = "organizations_url")]
    pub organizations_url: String,
    #[serde(rename = "repos_url")]
    pub repos_url: String,
    #[serde(rename = "events_url")]
    pub events_url: String,
    #[serde(rename = "received_events_url")]
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "site_admin")]
    pub site_admin: bool,
}

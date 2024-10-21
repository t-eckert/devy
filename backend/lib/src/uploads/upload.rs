use super::{statuses::Status, Changeset, Error, Result};
use crate::{
    date::Date,
    db,
    git::{Diff, Git},
    repositories::RepoRepository,
};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub previous_upload_id: Option<Uuid>,

    pub status: Status,
    pub repo: String,
    pub sha: String,
    pub logs: Option<Vec<String>>,

    // pub diff: Option<Vec<Diff>>,
    // pub changeset: Option<Changeset>,
    pub created_at: Date,
    pub updated_at: Date,
}

impl Upload {
    pub async fn clone_repo(mut self, db_conn: &db::Conn, git: &Git) -> Result<Self> {
        git.clone_repo(&self.dir(), &self.repo)?;
        self.sha = git.sha(&self.dir())?;

        Ok(self)
    }

    pub async fn previous_sha(&self, db_conn: &db::Conn) -> Result<Option<String>> {
        Ok(UploadRepository::get_previous(db_conn, self)
            .await?
            .map(|upload| upload.sha))
    }

    /// If the upload has an identified previous upload for the given repository.
    pub fn has_previous(&self) -> bool {
        self.previous_upload_id.is_some()
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_sha(&mut self, sha: &str) {
        self.sha = sha.to_string();
    }

    pub fn append_log(&mut self, log: &str) {
        if let Some(logs) = &mut self.logs {
            logs.push(log.to_string());
        } else {
            self.logs = Some(vec![log.to_string()]);
        }
    }

    pub fn dir(&self) -> String {
        format!("/tmp/{}", self.id)
    }
}

pub struct UploadRepository;

impl UploadRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        previous_upload_id: Option<Uuid>,
        repo: &str,
    ) -> db::Result<Uuid> {
        Ok(sqlx::query_file_as!(
            db::Id,
            "queries/insert_upload.sql",
            previous_upload_id,
            repo
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn update(db_conn: &db::Conn, upload: &Upload) -> db::Result<Uuid> {
        Ok(sqlx::query_file_as!(
            db::Id,
            "queries/update_upload.sql",
            upload.id,
            upload.previous_upload_id,
            upload.status.to_string(),
            upload.repo,
            upload.sha,
            &upload.logs.clone().unwrap_or_default(),
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Upload> {
        Ok(sqlx::query_file_as!(Upload, "queries/get_upload.sql", id)
            .fetch_one(db_conn)
            .await?)
    }

    pub async fn get_by_repo_url(db_conn: &db::Conn, repo_url: &str) -> db::Result<Upload> {
        Ok(
            sqlx::query_file_as!(Upload, "queries/get_upload_by_repo.sql", repo_url)
                .fetch_one(db_conn)
                .await?,
        )
    }

    /// Looks up the most recent upload for the given repository.
    pub async fn get_previous(db_conn: &db::Conn, upload: &Upload) -> db::Result<Option<Upload>> {
        Ok(
            sqlx::query_file_as!(Upload, "queries/get_previous_upload.sql", upload.repo)
                .fetch_optional(db_conn)
                .await?,
        )
    }
}

use super::{Error, Result};
use crate::{date::Date, db, git::Git, repositories::RepoRepository};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub previous_upload_id: Option<Uuid>,

    pub status: String,
    pub repo: String,
    pub sha: String,
    pub logs: Option<Vec<String>>,

    pub created_at: Date,
    pub updated_at: Date,
}

impl Upload {
    pub fn new(repo: &str, sha: Option<&str>) -> Self {
        Self {
            id: Uuid::new_v4(),
            previous_upload_id: None,
            status: "pending".to_string(),
            repo: repo.to_string(),
            sha: sha.unwrap_or("").to_string(),
            logs: None,
            created_at: Date::now(),
            updated_at: Date::now(),
        }
    }

    pub async fn verify(mut self, db_conn: &db::Conn) -> Result<Self> {
        tracing::info!("Verifying upload: {:?}", self);

        match RepoRepository::get_by_url(db_conn, &self.repo).await {
            Ok(_) => {
                self.set_status("verified");
                self.append_log("INFO: Upload verified");
                UploadRepository::update(db_conn, &self);
            }
            Err(err) => {
                self.set_status("failed");
                self.append_log(&format!("ERROR: {}", err));
                UploadRepository::update(db_conn, &self);
                return Err(Error::RepositoryNotFound(self.repo.clone()));
            }
        }

        Ok(self)
    }

    pub async fn receive(mut self, db_conn: &db::Conn) -> Result<Self> {
        self.set_status("received");
        self.append_log("INFO: Upload received by uploader");
        UploadRepository::update(db_conn, &self);

        // Set the previous upload if it exists.
        match UploadRepository::get_previous(db_conn, &self).await? {
            Some(previous_upload) => {
                self.previous_upload_id = Some(previous_upload.id);
            }
            None => {
                self.previous_upload_id = None;
            }
        }

        // Check if the upload is identical to the previous upload.
        if let Some(previous_sha) = self.previous_sha(db_conn).await? {
            if previous_sha == self.sha {
                self.set_status("skipped");
                self.append_log(
                    "INFO: Upload skipped because it is identical to the previous upload",
                );
                UploadRepository::update(db_conn, &self);
                return Ok(self);
            }
        }

        Ok(self)
    }

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

    fn set_status(&mut self, status: &str) {
        self.status = status.to_string();
    }

    fn set_sha(&mut self, sha: &str) {
        self.sha = sha.to_string();
    }

    fn append_log(&mut self, log: &str) {
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
            upload.status,
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

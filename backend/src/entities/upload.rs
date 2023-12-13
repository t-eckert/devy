use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

use super::error::Result;
use super::{Webhook, WebhookType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Uuid,
    pub previous_upload_id: Option<Uuid>,

    pub status: String,
    pub repo: String,
    pub logs: Option<Vec<String>>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUpload {
    pub previous_upload_id: Option<Uuid>,
    pub repo: String,
}

impl TryFrom<Webhook> for NewUpload {
    type Error = super::error::Error;
    fn try_from(value: Webhook) -> std::result::Result<Self, Self::Error> {
        match value.webhook_type {
            WebhookType::GitHubPushEvent => Ok(Self {
                previous_upload_id: None,
                repo: value.payload["repository"]["url"].to_string(),
            }),

            _ => Err(super::error::Error::Malformed(
                "Webhook does not have webhook_type of `webhook.github.push`".to_string(),
            )),
        }
    }
}

pub struct UploadRepository {}

impl UploadRepository {
    pub async fn insert(pool: &PgPool, upload: NewUpload) -> Result<Upload> {
        Ok(sqlx::query_file_as!(
            Upload,
            "queries/upload_insert.sql",
            upload.previous_upload_id,
            upload.repo,
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn set_status(pool: &PgPool, id: Uuid, status: &str) -> Result<Upload> {
        Ok(
            sqlx::query_file_as!(Upload, "queries/upload_set_status.sql", id, status)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn append_log(pool: &PgPool, id: Uuid, log: &str) -> Result<Upload> {
        Ok(
            sqlx::query_file_as!(Upload, "queries/upload_append_log.sql", id, log,)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Upload> {
        Ok(
            sqlx::query_file_as!(Upload, "queries/upload_get_by_id.sql", id,)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_by_repo(pool: &PgPool, repo: &str) -> Result<Upload> {
        Ok(
            sqlx::query_file_as!(Upload, "queries/upload_get_by_repo.sql", repo)
                .fetch_one(pool)
                .await?,
        )
    }

    pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Vec<Upload>> {
        Ok(
            sqlx::query_file_as!(Upload, "queries/upload_get_by_username.sql", username)
                .fetch_all(pool)
                .await?,
        )
    }
}

use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

use super::error::Result;
use super::{Webhook, WebhookType};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Upload {
    pub id: Option<String>,
    pub previous_upload_id: Option<String>,

    pub status: Option<String>,
    pub repo: Option<String>,
    pub logs: Option<Vec<String>>,

    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl Upload {
    pub fn new(previous_upload_id: Option<String>, repo: String) -> Self {
        Self {
            id: None,
            previous_upload_id,
            status: None,
            repo: Some(repo),
            logs: Some(vec![]),
            created_at: None,
            updated_at: None,
        }
    }

    pub async fn insert(self, pool: &PgPool) -> Result<Self> {
        let previous_upload_uuid = match &self.previous_upload_id {
            Some(previous_upload_id) => Uuid::parse_str(previous_upload_id).ok(),
            None => None,
        };

        Ok(sqlx::query_as!(
            Self,
            r#"
            INSERT INTO "upload" (previous_upload_id, repo, logs)
            VALUES ($1, $2, $3)
            RETURNING
                id::TEXT, previous_upload_id::TEXT, status, repo, logs,
                to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at; 
            "#,
            previous_upload_uuid,
            self.repo.unwrap(),
            &self.logs.unwrap_or(vec![])
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn set_status(self, pool: &PgPool, status: String) -> Result<Self> {
        let uuid = match &self.id {
            Some(id) => Uuid::parse_str(id).ok(),
            None => None,
        };

        Ok(sqlx::query_as!(
            Self,
            r#"
            UPDATE "upload"
            SET status=$2, updated_at=NOW()
            WHERE id=$1
            RETURNING
                id::TEXT, previous_upload_id::TEXT, status, repo, logs,
                to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at; 
            "#,
            uuid,
            status
        )
        .fetch_one(pool)
        .await?)
    }

    pub async fn log(self, pool: &PgPool, log: String) -> Result<Self> {
        let uuid = match &self.id {
            Some(id) => Uuid::parse_str(id).ok(),
            None => None,
        };

        Ok(sqlx::query_as!(
            Self,
            r#"
            UPDATE "upload"
            SET logs=ARRAY_APPEND(logs,$2), updated_at=NOW()
            WHERE id=$1
            RETURNING
                id::TEXT, previous_upload_id::TEXT, status, repo, logs,
                to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at; 
            "#,
            uuid,
            log
        )
        .fetch_one(pool)
        .await?)
    }

    #[allow(dead_code)]
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        Ok(sqlx::query_as!(
            Self,
            r#"
            SELECT 
                id::TEXT, previous_upload_id::TEXT, status, repo, logs,
                to_char(upload.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(upload.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at
            FROM "upload"
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(pool)
        .await?)
    }

    #[allow(dead_code)]
    pub async fn get_by_username(_pool: &PgPool, _username: String) -> Result<Vec<Self>> {
        Ok(vec![])
    }
}

impl TryFrom<Webhook> for Upload {
    type Error = super::error::Error;
    fn try_from(value: Webhook) -> std::result::Result<Self, Self::Error> {
        match value.webhook_type {
            WebhookType::GitHubPushEvent => Ok(Self::new(
                None,
                value.payload["repository"]["url"].to_string(),
            )),

            _ => Err(super::error::Error::Malformed(
                "Webhook does not have webhook_type of `webhook.github.push`".to_string(),
            )),
        }
    }
}

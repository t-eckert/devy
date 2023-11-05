use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

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

    pub async fn insert(self, pool: &PgPool) -> Result<Self, sqlx::Error> {
        let previous_upload_uuid = match &self.previous_upload_id {
            Some(previous_upload_id) => Uuid::parse_str(previous_upload_id).ok(),
            None => None,
        };

        sqlx::query_as!(
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
        .await
    }

    pub async fn set_status(self, pool: &PgPool, status: String) -> Result<Self, sqlx::Error> {
        let uuid = match &self.id {
            Some(id) => Uuid::parse_str(id).ok(),
            None => None,
        };

        sqlx::query_as!(
            Self,
            r#"
            UPDATE "upload"
            SET status=$2
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
        .await
    }
}

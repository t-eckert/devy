use super::*;
use crate::entities::webhook::{Webhook, WebhookType};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use sqlx::PgPool;

pub async fn insert(pool: &PgPool, upload: UploadForUpsert) -> Result<Upload> {
    Ok(sqlx::query_file_as!(
        Upload,
        "src/entities/upload/queries/insert.sql",
        upload.previous_upload_id,
        upload.repo,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn set_status(pool: &PgPool, id: Uuid, status: &str) -> Result<Upload> {
    Ok(sqlx::query_file_as!(
        Upload,
        "src/entities/upload/queries/set_status.sql",
        id,
        status
    )
    .fetch_one(pool)
    .await?)
}

pub async fn append_log(pool: &PgPool, id: Uuid, log: &str) -> Result<Upload> {
    Ok(sqlx::query_file_as!(
        Upload,
        "src/entities/upload/queries/append_log.sql",
        id,
        log,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Upload> {
    Ok(
        sqlx::query_file_as!(Upload, "src/entities/upload/queries/get_by_id.sql", id,)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_repo(pool: &PgPool, repo: &str) -> Result<Upload> {
    Ok(
        sqlx::query_file_as!(Upload, "src/entities/upload/queries/get_by_repo.sql", repo)
            .fetch_one(pool)
            .await?,
    )
}

// pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<Vec<Upload>> {
//     Ok(
//         sqlx::query_file_as!(Upload, "src/entities/upload/queries/get_by_username.sql", username)
//             .fetch_all(pool)
//             .await?,
//     )
// }

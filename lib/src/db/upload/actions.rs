use crate::db::{error::Result, Database};
use crate::entities::Upload;
use uuid::Uuid;

pub async fn insert(
    pool: &Database,
    previous_upload_id: Option<Uuid>,
    repo: String,
) -> Result<Upload> {
    Ok(sqlx::query_file_as!(
        Upload,
        "src/db/upload/queries/insert.sql",
        previous_upload_id,
        repo,
    )
    .fetch_one(pool)
    .await?)
}

pub async fn set_status(pool: &Database, id: Uuid, status: &str) -> Result<Upload> {
    Ok(
        sqlx::query_file_as!(Upload, "src/db/upload/queries/set_status.sql", id, status)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn set_previous(db: &Database, id: Uuid, previous_id: Uuid) -> Result<Upload> {
    Ok(sqlx::query_file_as!(
        Upload,
        "src/db/upload/queries/set_previous.sql",
        id,
        previous_id
    )
    .fetch_one(db)
    .await?)
}

pub async fn set_sha(db: &Database, id: Uuid, sha: String) -> Result<Upload> {
    Ok(
        sqlx::query_file_as!(Upload, "src/db/upload/queries/set_sha.sql", id, sha)
            .fetch_one(db)
            .await?,
    )
}

pub async fn append_log(pool: &Database, id: Uuid, log: &str) -> Result<Upload> {
    Ok(
        sqlx::query_file_as!(Upload, "src/db/upload/queries/append_log.sql", id, log,)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_id(pool: &Database, id: Uuid) -> Result<Upload> {
    Ok(
        sqlx::query_file_as!(Upload, "src/db/upload/queries/get_by_id.sql", id,)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_by_repo(pool: &Database, repo: &str) -> Result<Upload> {
    Ok(
        sqlx::query_file_as!(Upload, "src/db/upload/queries/get_by_repo.sql", repo)
            .fetch_one(pool)
            .await?,
    )
}

pub async fn get_previous(db: &Database, repo: &str) -> Result<Option<Upload>> {
    Ok(
        sqlx::query_file_as!(Upload, "src/db/upload/queries/get_previous.sql", repo)
            .fetch_optional(db)
            .await?,
    )
}

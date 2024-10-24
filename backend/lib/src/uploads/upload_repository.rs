use super::{Changeset, Status};
use crate::date::Date;
use crate::db;
use crate::uploads::Upload;
use sqlx::types::Json;
use uuid::Uuid;

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
            upload.diff,
            upload
                .changeset
                .clone()
                .map(|c| serde_json::to_value(c).unwrap_or_default()),
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Upload> {
        let row = sqlx::query_file!("queries/get_upload.sql", id)
            .fetch_one(db_conn)
            .await?;

        let changeset = match row.changeset {
            Some(c) => serde_json::from_value(c)?,
            None => None,
        };

        Ok(Upload {
            id: row.id,
            previous_upload_id: row.previous_upload_id,
            status: Status::from(row.status),
            repo: row.repo,
            sha: row.sha,
            logs: row.logs,
            diff: row.diff,
            changeset,
            created_at: Date::from(row.created_at),
            updated_at: Date::from(row.updated_at),
        })
    }

    pub async fn get_by_repo_url(db_conn: &db::Conn, repo_url: &str) -> db::Result<Upload> {
        let row = sqlx::query_file!("queries/get_upload_by_repo.sql", repo_url)
            .fetch_one(db_conn)
            .await?;

        let changeset = match row.changeset {
            Some(c) => serde_json::from_value(c)?,
            None => None,
        };

        Ok(Upload {
            id: row.id,
            previous_upload_id: row.previous_upload_id,
            status: Status::from(row.status),
            repo: row.repo,
            sha: row.sha,
            logs: row.logs,
            diff: row.diff,
            changeset,
            created_at: Date::from(row.created_at),
            updated_at: Date::from(row.updated_at),
        })
    }

    /// Looks up the most recent upload for the given repository.
    pub async fn get_previous(db_conn: &db::Conn, upload: &Upload) -> db::Result<Option<Upload>> {
        let row = sqlx::query_file!("queries/get_previous_upload.sql", upload.repo)
            .fetch_optional(db_conn)
            .await?;

        match row {
            None => Ok(None),
            Some(row) => {
                let changeset = match row.changeset {
                    Some(c) => serde_json::from_value(c)?,
                    None => None,
                };

                Ok(Some(Upload {
                    id: row.id,
                    previous_upload_id: row.previous_upload_id,
                    status: Status::from(row.status),
                    repo: row.repo,
                    sha: row.sha,
                    logs: row.logs,
                    diff: row.diff,
                    changeset,
                    created_at: Date::from(row.created_at),
                    updated_at: Date::from(row.updated_at),
                }))
            }
        }
    }
}

use crate::{
    date::Date,
    db,
    uploads::{Status, Upload},
};
use uuid::Uuid;

pub struct UploadRepository;

impl UploadRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        previous_upload_id: Option<Uuid>,
        blog_id: Uuid,
        repo: &str,
    ) -> db::Result<Uuid> {
        let res = sqlx::query_file_as!(
            db::Id,
            "queries/insert_upload.sql",
            previous_upload_id,
            blog_id,
            repo,
        )
        .fetch_one(db_conn)
        .await;

        dbg!(&res);

        let id = res?.id;
        dbg!(id);
        let notify = format!("NOTIFY upload, '{}';", &id);
        dbg!(&notify);
        sqlx::query(&notify).execute(db_conn).await?;

        Ok(id)
    }

    pub async fn update(db_conn: &db::Conn, upload: &Upload) -> db::Result<Uuid> {
        Ok(sqlx::query_file_as!(
            db::Id,
            "queries/update_upload.sql",
            upload.id,
            upload.previous_upload_id,
            upload.blog_id,
            upload.status.to_string(),
            upload.sha,
            &upload.logs.clone().unwrap_or_default(),
            upload.diff,
            upload
                .changeset
                .clone()
                .map(|c| serde_json::to_value(c).unwrap_or_default()),
            upload.uploader
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn claim(
        db_conn: &db::Conn,
        id: Uuid,
        uploader_name: &str,
    ) -> db::Result<(Upload, bool)> {
        let mut tx = db_conn.begin().await?;

        let row = sqlx::query_file!("queries/get_upload.sql", id)
            .fetch_one(&mut *tx)
            .await?;
        let changeset = match row.changeset {
            Some(c) => serde_json::from_value(c)?,
            None => None,
        };

        let mut upload = Upload {
            id: row.id,
            previous_upload_id: row.previous_upload_id,
            blog_id: row.blog_id,
            uploader: row.uploader,
            status: Status::from(row.status),
            sha: row.sha,
            logs: row.logs,
            diff: row.diff,
            changeset,
            created_at: Date::from(row.created_at),
            updated_at: Date::from(row.updated_at),
        };

        // If the upload is already claimed, return it as is.
        if upload.uploader.is_some() {
            tx.commit().await?;
            return Ok((upload, false));
        }

        upload.uploader = Some(uploader_name.to_string());
        let id = sqlx::query_file_as!(
            db::Id,
            "queries/update_upload.sql",
            upload.id,
            upload.previous_upload_id,
            upload.blog_id,
            upload.status.to_string(),
            upload.sha,
            &upload.logs.clone().unwrap_or_default(),
            upload.diff,
            upload
                .changeset
                .clone()
                .map(|c| serde_json::to_value(c).unwrap_or_default()),
            upload.uploader
        )
        .fetch_one(db_conn)
        .await?
        .id;

        tx.commit().await?;
        let claimed =
            upload.uploader.is_some() && upload.uploader.as_ref().unwrap() == uploader_name;

        Ok((UploadRepository::get(db_conn, id).await?, claimed))
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
            blog_id: row.blog_id,
            uploader: row.uploader,
            status: Status::from(row.status),
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
            blog_id: row.blog_id,
            uploader: row.uploader,
            status: Status::from(row.status),
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
        let row = sqlx::query_file!("queries/get_previous_upload.sql", upload.blog_id)
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
                    blog_id: row.blog_id,
                    uploader: row.uploader,
                    status: Status::from(row.status),
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

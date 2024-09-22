use crate::date::Date;
use crate::db;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Repo {
    pub id: Uuid,
    pub blog_id: Uuid,

    pub url: String,
    pub metadata: Value,

    pub created_at: Date,
    pub updated_at: Date,
}

pub struct RepoRepository;

impl RepoRepository {
    pub async fn insert(db_conn: &db::Conn, blog_id: Uuid, url: &str) -> db::Result<Uuid> {
        Ok(
            sqlx::query_file_as!(db::Id, "queries/insert_repo.sql", blog_id, url)
                .fetch_one(db_conn)
                .await?
                .id,
        )
    }

    pub async fn update(db_conn: &db::Conn, repo: Repo) -> db::Result<Uuid> {
        return Ok(sqlx::query_file_as!(
            db::Id,
            "queries/update_repo.sql",
            repo.id,
            repo.blog_id,
            repo.url,
            repo.metadata
        )
        .fetch_one(db_conn)
        .await?
        .id);
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Repo> {
        Ok(sqlx::query_file_as!(Repo, "queries/get_repo.sql", id)
            .fetch_one(db_conn)
            .await?)
    }

    pub async fn get_by_url(db_conn: &db::Conn, url: &str) -> db::Result<Repo> {
        return Ok(
            sqlx::query_file_as!(Repo, "queries/get_repo_by_url.sql", url)
                .fetch_one(db_conn)
                .await?,
        );
    }
}

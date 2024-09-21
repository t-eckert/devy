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
    pub async fn insert() -> Uuid {
        return Uuid::new_v4();
    }

    pub async fn upsert(db_conn: &db::Conn, blog_id: Uuid, url: &str) -> db::Result<Uuid> {
        return Ok(Uuid::new_v4());
    }

    pub async fn set_metadata(db_conn: &db::Conn, id: Uuid, metadata: Value) -> db::Result<()> {
        return Ok(());
    }

    pub async fn get_by_url(db_conn: &db::Conn, url: &str) -> db::Result<Repo> {
        return Ok(Repo {
            id: Uuid::new_v4(),
            blog_id: Uuid::new_v4(),
            url: "url".to_string(),
            metadata: Value::Null,
            created_at: Date::now(),
            updated_at: Date::now(),
        });
    }
}

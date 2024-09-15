use crate::db;
use crate::posts::Entry;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const PAGE_SIZE: i64 = 500;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Feed {
    pub config: FeedConfig,
    pub page: u32,
    pub count: usize,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FeedConfig {
    pub id: String,
    pub name: String,
}

impl FeedConfig {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
        }
    }
}

pub struct FeedRepository;

impl FeedRepository {
    pub async fn get_recent(db_conn: &db::Conn, page: u32) -> db::Result<Feed> {
        let limit = PAGE_SIZE;
        let offset = offset(page as i64);

        let entries = sqlx::query_file_as!(
            Entry,
            "queries/get_feed_entries_for_recent.sql",
            limit,
            offset
        )
        .fetch_all(db_conn)
        .await?;

        Ok(Feed {
            config: FeedConfig::new("recent", "Recent"),
            page,
            count: entries.len(),
            entries,
        })
    }

    pub async fn get_popular(db_conn: &db::Conn, page: u32) -> db::Result<Feed> {
        let limit = PAGE_SIZE;
        let offset = offset(page as i64);

        let entries = sqlx::query_file_as!(
            Entry,
            "queries/get_feed_entries_for_popular.sql",
            limit,
            offset
        )
        .fetch_all(db_conn)
        .await?;

        Ok(Feed {
            config: FeedConfig::new("popular", "Popular"),
            page,
            count: entries.len(),
            entries,
        })
    }

    pub async fn get_following(
        db_conn: &db::Conn,
        profile_id: Uuid,
        page: u32,
    ) -> db::Result<Feed> {
        let limit = PAGE_SIZE;
        let offset = offset(page as i64);

        let entries = sqlx::query_file_as!(
            Entry,
            "queries/get_feed_entries_for_following.sql",
            limit,
            offset,
            profile_id
        )
        .fetch_all(db_conn)
        .await?;

        Ok(Feed {
            config: FeedConfig::new("following", "Following"),
            page,
            count: entries.len(),
            entries,
        })
    }
}

fn offset(page: i64) -> i64 {
    (page - 1) * PAGE_SIZE
}

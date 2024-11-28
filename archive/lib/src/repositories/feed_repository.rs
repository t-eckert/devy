use crate::db;
use crate::posts::{Entry, Feed, FeedConfig};
use uuid::Uuid;

const PAGE_SIZE: i64 = 500;

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
            page_size: PAGE_SIZE,
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
            page_size: PAGE_SIZE,
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
            page_size: PAGE_SIZE,
            count: entries.len(),
            entries,
        })
    }
}

fn offset(page: i64) -> i64 {
    (page - 1) * PAGE_SIZE
}

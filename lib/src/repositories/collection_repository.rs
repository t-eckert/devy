use crate::db;
use crate::posts::{Collection, CollectionConfig, Entry};
use uuid::Uuid;

const PAGE_SIZE: i64 = 500;

pub struct CollectionRepository;

impl CollectionRepository {
    /// Retrieve the collection of all entries liked by the profile.
    pub async fn get_liked(
        db_conn: &db::Conn,
        profile_id: Uuid,
        page: u32,
    ) -> db::Result<Collection> {
        let limit = PAGE_SIZE;
        let offset = offset(page as i64);

        let entries = sqlx::query_file_as!(
            Entry,
            "queries/get_collection_entries_for_liked.sql",
            profile_id,
            limit,
            offset
        )
        .fetch_all(db_conn)
        .await?;

        Ok(Collection {
            config: CollectionConfig::new("liked", "Liked"),
            page,
            page_size: PAGE_SIZE,
            count: entries.len(),
            entries,
        })
    }

    /// Retrieve the colleciton of all entries bookmarked by the profile.
    pub async fn get_bookmarked(
        db_conn: &db::Conn,
        profile_id: Uuid,
        page: u32,
    ) -> db::Result<Collection> {
        let limit = PAGE_SIZE;
        let offset = offset(page as i64);

        let entries = sqlx::query_file_as!(
            Entry,
            "queries/get_collection_entries_for_bookmarked.sql",
            profile_id,
            limit,
            offset
        )
        .fetch_all(db_conn)
        .await?;

        Ok(Collection {
            config: CollectionConfig::new("bookmarked", "Bookmarked"),
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

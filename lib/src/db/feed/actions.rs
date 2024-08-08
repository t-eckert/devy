use crate::db::{error::Result, Database};
use crate::entities::{Entry, Feed, FeedConfig};

pub async fn get_recent(db: &Database) -> Result<Feed> {
    Ok(Feed {
        feed_config: FeedConfig::new("recent", "Recent Posts"),
        entries: sqlx::query_file_as!(Entry, "src/db/feed/queries/get_recent.sql")
            .fetch_all(db)
            .await?,
    })
}

pub async fn get_popular(db: &Database) -> Result<Feed> {
    Ok(Feed {
        feed_config: FeedConfig::new("popular", "Popular Posts"),
        entries: sqlx::query_file_as!(Entry, "src/db/feed/queries/get_popular.sql")
            .fetch_all(db)
            .await?,
    })
}

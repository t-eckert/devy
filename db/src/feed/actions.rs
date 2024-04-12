use crate::{error::Result, Database};
use entities::{Entry, Feed, FeedConfig};

pub async fn get_recent(db: &Database) -> Result<Feed> {
    Ok(Feed {
        feed_config: FeedConfig::new("recent", "Recent"),
        entries: sqlx::query_file_as!(Entry, "src/feed/queries/get_recent.sql")
            .fetch_all(db)
            .await?,
    })
}

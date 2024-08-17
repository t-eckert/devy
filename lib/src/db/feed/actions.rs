use uuid::Uuid;

use crate::db::{error::Result, Database};
use crate::entities::{Entry, Feed, FeedConfig};

pub async fn get_recent(db: &Database) -> Result<Feed> {
    Ok(Feed {
        feed_config: FeedConfig::new("recent", "Recent"),
        entries: sqlx::query_file_as!(Entry, "src/db/feed/queries/get_recent.sql")
            .fetch_all(db)
            .await?,
    })
}

pub async fn get_popular(db: &Database) -> Result<Feed> {
    Ok(Feed {
        feed_config: FeedConfig::new("popular", "Popular"),
        entries: sqlx::query_file_as!(Entry, "src/db/feed/queries/get_popular.sql")
            .fetch_all(db)
            .await?,
    })
}

pub async fn get_following(db: &Database, profile_id: Uuid) -> Result<Feed> {
    Ok(Feed {
        feed_config: FeedConfig::new("following", "Following"),
        entries: sqlx::query_file_as!(Entry, "src/db/feed/queries/get_following.sql")
            .fetch_all(db)
            .await?,
    })
}

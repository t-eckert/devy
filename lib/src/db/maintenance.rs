use super::{Conn, Result};

/// Update all posts with the like count.
pub async fn update_all_posts_with_like_count(db_conn: &Conn) -> Result<()> {
    sqlx::query_file!("queries/update_all_posts_with_like_count.sql")
        .execute(db_conn)
        .await?;

    Ok(())
}

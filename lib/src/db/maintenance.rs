use super::{error::Result, DBConn};

pub async fn update_all_posts_with_like_count(db_conn: &DBConn) -> Result<()> {
    sqlx::query_file!("queries/update_all_posts_with_like_count.sql")
        .execute(db_conn)
        .await?;

    Ok(())
}

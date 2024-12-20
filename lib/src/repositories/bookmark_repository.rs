use crate::db;
use crate::posts::Bookmark;
use uuid::Uuid;

pub struct BookmarkRepository;

impl BookmarkRepository {
    pub async fn insert(db_conn: &db::Conn, profile_id: Uuid, post_id: Uuid) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/insert_bookmark.sql", profile_id, post_id)
            .execute(db_conn)
            .await?;
        Ok(())
    }

    pub async fn get(db_conn: &db::Conn, profile_id: Uuid, post_id: Uuid) -> db::Result<Bookmark> {
        Ok(
            sqlx::query_file_as!(Bookmark, "queries/get_bookmark.sql", profile_id, post_id)
                .fetch_one(db_conn)
                .await?,
        )
    }

    pub async fn get_by_username(
        db_conn: &db::Conn,
        username: &String,
    ) -> db::Result<Vec<Bookmark>> {
        Ok(
            sqlx::query_file_as!(Bookmark, "queries/get_bookmarks_by_username.sql", username)
                .fetch_all(db_conn)
                .await?,
        )
    }

    pub async fn delete(db_conn: &db::Conn, profile_id: Uuid, post_id: Uuid) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/delete_bookmark.sql", profile_id, post_id)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

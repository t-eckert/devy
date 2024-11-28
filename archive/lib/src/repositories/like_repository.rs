use crate::db;
use crate::posts::Like;
use uuid::Uuid;

pub struct LikeRepository;

impl LikeRepository {
    pub async fn insert(db_conn: &db::Conn, profile_id: Uuid, post_id: Uuid) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/insert_like.sql", profile_id, post_id)
            .execute(db_conn)
            .await?;
        Ok(())
    }

    pub async fn get(db_conn: &db::Conn, profile_id: Uuid, post_id: Uuid) -> db::Result<Like> {
        Ok(
            sqlx::query_file_as!(Like, "queries/get_like.sql", profile_id, post_id)
                .fetch_one(db_conn)
                .await?,
        )
    }

    pub async fn get_by_username(db_conn: &db::Conn, username: &String) -> db::Result<Vec<Like>> {
        Ok(
            sqlx::query_file_as!(Like, "queries/get_likes_by_username.sql", username)
                .fetch_all(db_conn)
                .await?,
        )
    }

    pub async fn get_by_profile_id(db_conn: &db::Conn, profile_id: Uuid) -> db::Result<Vec<Like>> {
        Ok(
            sqlx::query_file_as!(Like, "queries/get_likes_by_profile_id.sql", profile_id)
                .fetch_all(db_conn)
                .await?,
        )
    }

    pub async fn delete(db_conn: &db::Conn, profile_id: Uuid, post_id: Uuid) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/delete_like.sql", profile_id, post_id)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

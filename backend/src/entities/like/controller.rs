use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::Like;

pub struct LikeController {}

impl LikeController {
    pub async fn upsert(mut db: Connection<DB>, like: Like) -> Option<Like> {
        let inserted = sqlx::query_as!(
            Like,
            r#"
            INSERT INTO "like" (profile_id, post_id)
            VALUES ($1, $2)
            ON CONFLICT (profile_id, post_id)
            DO UPDATE SET profile_id = $1, post_id = $2
            RETURNING profile_id, post_id;
            "#,
            like.profile_id,
            like.post_id
        )
        .fetch_one(&mut *db)
        .await;

        match inserted {
            Ok(like) => Some(like),
            Err(err) => {
                println!("Error: {}", err);
                None
            }
        }
    }

    pub async fn delete(mut db: Connection<DB>, profile_id: i32, post_id: i32) -> Option<Like> {
        let deleted = sqlx::query_as!(
            Like,
            r#"
            DELETE FROM "like"
            WHERE profile_id = $1 AND post_id = $2
            RETURNING profile_id, post_id;
            "#,
            profile_id,
            post_id
        )
        .fetch_one(&mut *db)
        .await;

        match deleted {
            Ok(like) => Some(like),
            Err(err) => {
                println!("Error: {}", err);
                None
            }
        }
    }
}

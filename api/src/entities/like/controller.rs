use rocket_db_pools::Connection;
use uuid::{uuid, Uuid};

use crate::db::DB;
use crate::entities::Like;

pub struct LikeController {}

impl LikeController {
    pub async fn upsert(mut db: Connection<DB>, like: Like) -> Option<Like> {
        // Profile and post ids must be present and valid uuids.
        let (profile_id, post_id) = match (like.profile_id, like.post_id) {
            (Some(profile_id), Some(post_id)) => {
                match (
                    Uuid::parse_str(profile_id.as_str()),
                    Uuid::parse_str(post_id.as_str()),
                ) {
                    (Ok(profile_id), Ok(post_id)) => (profile_id, post_id),
                    _ => return None,
                }
            }
            _ => return None,
        };

        let inserted = sqlx::query_as!(
            Like,
            r#"
            INSERT INTO "like" (profile_id, post_id)
            VALUES ($1, $2)
                ON CONFLICT (profile_id, post_id)
                DO UPDATE SET profile_id = $1, post_id = $2
            RETURNING profile_id::TEXT, post_id::TEXT;
            "#,
            profile_id,
            post_id
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

    pub async fn delete(
        mut db: Connection<DB>,
        profile_id: String,
        post_id: String,
    ) -> Option<Like> {
        let (profile, post) = match (
            Uuid::parse_str(profile_id.as_str()),
            Uuid::parse_str(post_id.as_str()),
        ) {
            (Ok(profile), Ok(post)) => (profile, post),
            _ => return None,
        };

        let deleted = sqlx::query_as!(
            Like,
            r#"
            DELETE FROM "like"
            WHERE profile_id = $1 AND post_id = $2
            RETURNING profile_id::TEXT, post_id::TEXT;
            "#,
            profile,
            post
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

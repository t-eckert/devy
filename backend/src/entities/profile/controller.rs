use rocket_db_pools::Connection;
use std::sync::Arc;

use crate::db::DB;

use super::model::Profile;

pub struct ProfileController {
    conn: Arc<Connection<DB>>,
}

impl ProfileController {
    pub fn new(conn: Arc<Connection<DB>>) -> Self {
        ProfileController { conn }
    }

    pub async fn insert_profile(&mut self, profile: Profile) -> Option<Profile> {
        sqlx::query_as!(
            Profile,
            r#"
            INSERT INTO profile (display_name, avatar_url)
            VALUES ($1, $2)
            RETURNING 
                display_name,
                to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                avatar_url;
            "#,
            profile.display_name,
            profile.avatar_url,
        )
        .fetch_one(&mut *self.conn)
        .await
        .ok()
    }

    pub async fn get_by_slug(&mut self, slug: String) -> Option<Profile> {
        sqlx::query_file_as!(Profile, "queries/profile_get_by_slug.sql", slug)
            .fetch_one(&mut *self.conn)
            .await
            .ok()
    }
}

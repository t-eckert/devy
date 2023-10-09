use rocket_db_pools::Connection;
use sqlx::types::Uuid;

use crate::db::DB;

use super::model::Profile;

pub struct ProfileController {}

impl ProfileController {

    pub async fn get_by_id(conn: &mut Connection<DB>, id: String) -> Option<Profile> {
        let uuid = Uuid::parse_str(&id).ok();

        sqlx::query_as!(
            Profile,
            r#"
            SELECT 
                user_id::TEXT, display_name,
                to_char(profile.created_at, 'YYYY-MM-DDThh:mm:ss.ss') AS created_at,
                to_char(profile.updated_at, 'YYYY-MM-DDThh:mm:ss.ss') AS updated_at,
                avatar_url
            FROM profile 
            WHERE id = $1"#,
            uuid
        )
        .fetch_one(&mut **conn)
        .await
        .ok()
    }

    pub async fn get_by_username(conn: &mut Connection<DB>, username: String) -> Option<Profile> {
        sqlx::query_file_as!(Profile, "queries/profile_get_by_slug.sql", username)
            .fetch_one(&mut **conn)
            .await
            .ok()
    }
}

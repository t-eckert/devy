use rocket_db_pools::Connection;

use crate::db::DB;

use super::model::Profile;

pub struct ProfileController {}

impl ProfileController {
    pub async fn get_by_slug(mut db: Connection<DB>, slug: String) -> Option<Profile> {
        sqlx::query_file_as!(Profile, "queries/profile_get_by_slug.sql", slug)
            .fetch_one(&mut *db)
            .await
            .ok()
    }
}

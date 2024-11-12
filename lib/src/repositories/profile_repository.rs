use crate::db;
use crate::profile::Profile;
use uuid::Uuid;

pub struct ProfileRepository;

impl ProfileRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        user_id: Uuid,
        display_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>,
        website: Option<String>,
    ) -> db::Result<Uuid> {
        Ok(sqlx::query_file_as!(
            db::Id,
            "queries/insert_profile.sql",
            user_id,
            display_name,
            avatar_url,
            bio,
            website,
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn update(db_conn: &db::Conn, profile: Profile) -> db::Result<Uuid> {
        Ok(sqlx::query_file_as!(
            db::Id,
            "queries/update_profile.sql",
            profile.id,
            profile.user_id,
            profile.display_name,
            profile.avatar_url,
            profile.bio,
            profile.website_url,
            profile.twitter_username,
            profile.github_username,
            profile.status.to_string(),
            profile.visibility.to_string(),
            profile.is_deleted,
            profile.is_locked,
            profile.is_featured,
            profile.is_bot,
            profile.is_sponsor,
            profile.bluesky_url,
            profile.linkedin_url,
        )
        .fetch_one(db_conn)
        .await?
        .id)
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Profile> {
        Ok(sqlx::query_file_as!(Profile, "queries/get_profile.sql", id)
            .fetch_one(db_conn)
            .await?)
    }

    pub async fn get_by_username(db_conn: &db::Conn, username: &String) -> db::Result<Profile> {
        Ok(
            sqlx::query_file_as!(Profile, "queries/get_profile_by_username.sql", username)
                .fetch_one(db_conn)
                .await?,
        )
    }

    pub async fn delete(db_conn: &db::Conn, id: Uuid) -> db::Result<()> {
        let _ = sqlx::query_file!("queries/delete_profile.sql", id)
            .execute(db_conn)
            .await?;

        Ok(())
    }
}

use crate::db;
use crate::{date::Date, github};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Profile {
    pub id: Uuid,
    pub user_id: Uuid,

    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: Option<String>,
    pub website_url: Option<String>,
    pub twitter_username: Option<String>,
    pub github_username: Option<String>,

    pub status: String,
    pub visibility: String,

    pub is_deleted: bool,
    pub is_locked: bool,
    pub is_featured: bool,
    pub is_bot: bool,
    pub is_sponsor: bool,

    pub created_at: Date,
    pub updated_at: Date,
}

impl Profile {
    pub fn update_from_github_user(&mut self, github_user: github::GitHubUser) {
        self.display_name = github_user.name;
        self.avatar_url = github_user.avatar_url;
        self.bio = github_user.bio;
        self.website_url = github_user.blog;
        self.twitter_username = github_user.twitter_username;
        self.github_username = github_user.login;
    }
}

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
            profile.status,
            profile.visibility,
            profile.is_deleted,
            profile.is_locked,
            profile.is_featured,
            profile.is_bot,
            profile.is_sponsor,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::users;

    #[sqlx::test]
    async fn test_insert_fails_if_user_does_not_exist(db_conn: db::Conn) {
        let result = ProfileRepository::insert(
            &db_conn,
            Uuid::new_v4(),
            Some("display_name".to_string()),
            Some("avatar_url".to_string()),
            Some("bio".to_string()),
            Some("website".to_string()),
        )
        .await;

        assert!(result.is_err());
    }
}

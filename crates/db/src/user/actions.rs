use crate::entities::error::{Error, Result};
use sqlx::PgPool;

pub async fn upsert(pool: &PgPool, user: UserForUpsert) -> Result<User> {
    Ok(sqlx::query_file_as!(
        User,
        "src/entities/user/queries/upsert.sql",
        user.username,
        user.email,
        user.github_username,
        user.role,
        user.status
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_by_username(pool: &PgPool, username: &str) -> Result<User> {
    Ok(sqlx::query_file_as!(
        User,
        "src/entities/user/queries/get_by_username.sql",
        username
    )
    .fetch_one(pool)
    .await?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_upsert_with_new_user_sets_correct_defaults(pool: PgPool) {
        let username = "taylor-schwift".to_string();
        let email = Some("t-swizzle@gmizzle.com".to_string());
        let github_username = "t-swizzle".to_string();

        let given = UserForUpsert {
            username: username.clone(),
            email: email.clone(),
            github_username: Some(github_username.clone()),
            role: None,
            status: None,
        };
        let actual = upsert(&pool, given).await.unwrap();

        let expected_username = username;
        let expected_email = email;
        let expected_github_username = github_username;
        let expected_role = "user".to_string();
        let expected_status = "active".to_string();

        assert_eq!(actual.username, expected_username);
        assert_eq!(actual.email, expected_email);
        assert_eq!(actual.github_username, Some(expected_github_username));
        assert_eq!(actual.role, expected_role);
        assert_eq!(actual.status, expected_status);
        assert_eq!(actual.created_at, actual.updated_at);
    }

    #[sqlx::test]
    async fn test_upsert_with_existing_user_to_change_email(pool: PgPool) {
        let username = "taylor-schwift".to_string();
        let initial_email = Some("t-swizzle@gmizzle.com".to_string());
        let modified_email = Some("ts@gmizzle.com".to_string());
        let github_username = "t-swizzle".to_string();

        let initial_given = UserForUpsert {
            username: username.clone(),
            email: initial_email.clone(),
            github_username: Some(github_username.clone()),
            role: None,
            status: None,
        };
        let _ = upsert(&pool, initial_given).await.unwrap();

        let modified_given = UserForUpsert {
            username: username.clone(),
            email: modified_email.clone(),
            github_username: Some(github_username.clone()),
            role: None,
            status: None,
        };
        let actual = upsert(&pool, modified_given).await.unwrap();

        let expected_username = username;
        let expected_email = modified_email;
        let expected_github_username = github_username;
        let expected_role = "user".to_string();
        let expected_status = "active".to_string();

        assert_eq!(actual.username, expected_username);
        assert_eq!(actual.email, expected_email);
        assert_eq!(actual.github_username, Some(expected_github_username));
        assert_eq!(actual.role, expected_role);
        assert_eq!(actual.status, expected_status);
        assert_eq!(actual.created_at, actual.updated_at);
    }

    #[sqlx::test]
    async fn test_upsert_from_github_user_with_new_user(pool: PgPool) {
        let email = Some("t-swizzle@gmizzle.com".to_string());
        let github_username = "t-swizzle".to_string();

        let given = GitHubUser {
            login: Some(github_username.clone()),
            id: None,
            node_id: None,
            avatar_url: None,
            gravatar_id: None,
            url: None,
            html_url: None,
            followers_url: None,
            following_url: None,
            gists_url: None,
            starred_url: None,
            subscriptions_url: None,
            organizations_url: None,
            repos_url: None,
            events_url: None,
            received_events_url: None,
            user_type: None,
            site_admin: None,
            name: None,
            company: None,
            blog: None,
            location: None,
            email: email.clone(),
            hireable: None,
            bio: None,
            twitter_username: None,
            public_repos: None,
            public_gists: None,
            followers: None,
            following: None,
            created_at: None,
            updated_at: None,
        };
        let actual = upsert_from_github_user(&pool, given).await.unwrap();

        let expected_username = github_username.clone();
        let expected_email = email;
        let expected_github_username = github_username;
        let expected_role = "user".to_string();
        let expected_status = "active".to_string();

        assert_eq!(actual.username, expected_username);
        assert_eq!(actual.email, expected_email);
        assert_eq!(actual.github_username, Some(expected_github_username));
        assert_eq!(actual.role, expected_role);
        assert_eq!(actual.status, expected_status);
        assert_eq!(actual.created_at, actual.updated_at);
    }

    #[sqlx::test]
    async fn test_upsert_from_github_user_with_new_user_without_email(pool: PgPool) {
        let email = None;
        let github_username = "t-swizzle".to_string();

        let given = GitHubUser {
            login: Some(github_username.clone()),
            id: None,
            node_id: None,
            avatar_url: None,
            gravatar_id: None,
            url: None,
            html_url: None,
            followers_url: None,
            following_url: None,
            gists_url: None,
            starred_url: None,
            subscriptions_url: None,
            organizations_url: None,
            repos_url: None,
            events_url: None,
            received_events_url: None,
            user_type: None,
            site_admin: None,
            name: None,
            company: None,
            blog: None,
            location: None,
            email: None,
            hireable: None,
            bio: None,
            twitter_username: None,
            public_repos: None,
            public_gists: None,
            followers: None,
            following: None,
            created_at: None,
            updated_at: None,
        };
        let actual = upsert_from_github_user(&pool, given).await.unwrap();

        let expected_username = github_username.clone();
        let expected_email = email;
        let expected_github_username = github_username;
        let expected_role = "user".to_string();
        let expected_status = "active".to_string();

        assert_eq!(actual.username, expected_username);
        assert_eq!(actual.email, expected_email);
        assert_eq!(actual.github_username, Some(expected_github_username));
        assert_eq!(actual.role, expected_role);
        assert_eq!(actual.status, expected_status);
        assert_eq!(actual.created_at, actual.updated_at);
    }

    #[sqlx::test]
    async fn test_get_by_username_with_existing_user(pool: PgPool) {
        let username = "taylor-schwift".to_string();
        let email = Some("t-swizzle@gmizzle.com".to_string());
        let github_username = "t-swizzle".to_string();

        let given = UserForUpsert {
            username: username.clone(),
            email: email.clone(),
            github_username: Some(github_username.clone()),
            role: None,
            status: None,
        };
        let _ = upsert(&pool, given).await.unwrap();

        let actual = get_by_username(&pool, &username).await.unwrap();

        let expected_username = username;
        let expected_email = email;
        let expected_github_username = github_username;
        let expected_role = "user".to_string();
        let expected_status = "active".to_string();

        assert_eq!(actual.username, expected_username);
        assert_eq!(actual.email, expected_email);
        assert_eq!(actual.github_username, Some(expected_github_username));
        assert_eq!(actual.role, expected_role);
        assert_eq!(actual.status, expected_status);
        assert_eq!(actual.created_at, actual.updated_at);
    }
}

use crate::db::{error::Result, DBConn};
use crate::entities::User;

pub async fn upsert(
    db_conn: &DBConn,
    username: String,
    email: Option<String>,
    github_username: Option<String>,
    status: Option<String>,
) -> Result<User> {
    Ok(sqlx::query_file_as!(
        User,
        "src/db/user/queries/upsert.sql",
        username,
        email,
        github_username,
        status
    )
    .fetch_one(db_conn)
    .await?)
}

pub async fn set_role_by_username(db_conn: &DBConn, username: &str, role: &str) -> Result<()> {
    // TODO: I can get rid of this and just use "upsert" from the controller.
    sqlx::query_file_as!(User, "src/db/user/queries/set_role_by_username.sql", username, role)
        .fetch_one(db_conn)
        .await?;
    Ok(())
}

pub async fn count(db: &DBConn) -> Result<i64> {
    Ok(sqlx::query_file!("src/db/user/queries/count.sql")
        .fetch_one(db)
        .await?
        .count
        .unwrap())
}

pub async fn get_by_username(db_conn: &DBConn, username: &str) -> Result<User> {
    Ok(
        sqlx::query_file_as!(User, "src/db/user/queries/get_by_username.sql", username)
            .fetch_one(db_conn)
            .await?,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[sqlx::test]
    async fn test_upsert_with_new_user_sets_correct_defaults(db_conn: DBConn) {
        let username = "taylor-schwift".to_string();
        let email = Some("t-swizzle@gmizzle.com".to_string());
        let github_username = "t-swizzle".to_string();

        let actual = upsert(
            &db_conn,
            username.clone(),
            email.clone(),
            Some(github_username.clone()),
            None,
        )
        .await
        .unwrap();

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
    async fn test_upsert_with_existing_user_to_change_email(db_conn: DBConn) {
        let username = "taylor-schwift".to_string();
        let initial_email = Some("t-swizzle@gmizzle.com".to_string());
        let modified_email = Some("ts@gmizzle.com".to_string());
        let github_username = "t-swizzle".to_string();

        let _ = upsert(
            &db_conn,
            username.clone(),
            initial_email.clone(),
            Some(github_username.clone()),
            None,
        )
        .await
        .unwrap();

        let actual = upsert(
            &db_conn,
            username.clone(),
            modified_email.clone(),
            Some(github_username.clone()),
            None,
        )
        .await
        .unwrap();

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
    async fn test_get_by_username_with_existing_user(db_conn: DBConn) {
        let username = "taylor-schwift".to_string();
        let email = Some("t-swizzle@gmizzle.com".to_string());
        let github_username = "t-swizzle".to_string();

        let _ = upsert(
            &db_conn,
            username.clone(),
            email.clone(),
            Some(github_username.clone()),
            None,
        )
        .await
        .unwrap();

        let actual = get_by_username(&db_conn, &username).await.unwrap();

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

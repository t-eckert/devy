use uuid::Uuid;
use crate::Database;

pub async fn upsert(
    db: &Database, profile_id: Uuid, name: String, slug: String, description Option<String>
) -> Result<Blog> {
    Ok(sqlx::query_file_as!(
        Blog,
        "src/entities/blog/queries/upsert.sql",
        profile_id,
        blog.name,
        blog.slug,
        blog.description,
    )
    .fetch_one(db)
    .await?)
}

pub async fn get_by_id(db: &Database, id: Uuid) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/entities/blog/queries/get_by_id.sql", id)
            .fetch_one(db)
            .await?,
    )
}

pub async fn get_by_slug(db: &Database, slug: String) -> Result<Blog> {
    Ok(
        sqlx::query_file_as!(Blog, "src/entities/blog/queries/get_by_slug.sql", slug)
            .fetch_one(db)
            .await?,
    )
}

pub async fn get_by_username(db: &Database, username: String) -> Result<Vec<Blog>> {
    Ok(sqlx::query_file_as!(
        Blog,
        "src/entities/blog/queries/get_by_username.sql",
        username
    )
    .fetch_all(db)
    .await?)
}

pub async fn delete_by_slug(db: &Database, slug: String) -> Result<()> {
    sqlx::query_file!("src/entities/blog/queries/delete_by_slug.sql", slug)
        .execute(db)
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::{profile, user};

    #[sqlx::test]
    async fn test_upsert_with_new_blog_sets_correct_defaults(db: Database) {
        // Create user to own profile.
        let username = "llavascript".to_string();
        let user = user::UserForUpsert::new(username.clone(), None);
        let user = user::upsert(&db, user).await.unwrap();

        // Create profile to own blog.
        let display_name = "Llava Script".to_string();
        let profile = profile::ProfileForUpsert::new(user.id.to_string(), display_name, None);
        let profile = profile::upsert(&db, profile).await.unwrap();

        // Test blog upsert.
        let profile_id = Uuid::parse_str(&profile.id.unwrap()).unwrap();
        let name = "My Favorite JS Frameworks".to_string();
        let slug = "my-favorite-js-frameworks".to_string();
        let description = "A blog about my favorite JS frameworks".to_string();

        let given = BlogForUpsert::new(name.clone(), slug.clone(), profile_id)
            .with_description(description.clone());
        let actual = upsert(&db, given).await.unwrap();

        let expected_profile_id = profile_id;
        let expected_name = name;
        let expected_slug = slug;
        let expected_description = description;

        assert_eq!(actual.profile_id, expected_profile_id);
        assert_eq!(actual.name, expected_name);
        assert_eq!(actual.slug, expected_slug);
        assert_eq!(actual.description, Some(expected_description));
    }

    #[sqlx::test]
    async fn test_get_by_id(db: Database) {
        // Create user to own profile.
        let username = "llavascript".to_string();
        let user = user::UserForUpsert::new(username.clone(), None);
        let user = user::upsert(&db, user).await.unwrap();

        // Create profile to own blog.
        let display_name = "Llava Script".to_string();
        let profile = profile::ProfileForUpsert::new(user.id.to_string(), display_name, None);
        let profile = profile::upsert(&db, profile).await.unwrap();

        // Create a blog.
        let profile_id = Uuid::parse_str(&profile.id.unwrap()).unwrap();
        let name = "My Favorite JS Frameworks".to_string();
        let slug = "my-favorite-js-frameworks".to_string();
        let description = "A blog about my favorite JS frameworks".to_string();

        let given = BlogForUpsert::new(name.clone(), slug.clone(), profile_id)
            .with_description(description.clone());
        let inserted = upsert(&db, given).await.unwrap();

        let actual = get_by_id(&db, inserted.id).await.unwrap();

        let expected_profile_id = profile_id;
        let expected_name = name;
        let expected_slug = slug;
        let expected_description = description;

        assert_eq!(actual.profile_id, expected_profile_id);
        assert_eq!(actual.name, expected_name);
        assert_eq!(actual.slug, expected_slug);
        assert_eq!(actual.description, Some(expected_description));
    }

    #[sqlx::test]
    async fn test_get_by_slug(db: Database) {
        // Create user to own profile.
        let username = "llavascript".to_string();
        let user = user::UserForUpsert::new(username.clone(), None);
        let user = user::upsert(&db, user).await.unwrap();

        // Create profile to own blog.
        let display_name = "Llava Script".to_string();
        let profile = profile::ProfileForUpsert::new(user.id.to_string(), display_name, None);
        let profile = profile::upsert(&db, profile).await.unwrap();

        // Create a blog.
        let profile_id = Uuid::parse_str(&profile.id.unwrap()).unwrap();
        let name = "My Favorite JS Frameworks".to_string();
        let slug = "my-favorite-js-frameworks".to_string();
        let description = "A blog about my favorite JS frameworks".to_string();

        let given = BlogForUpsert::new(name.clone(), slug.clone(), profile_id)
            .with_description(description.clone());
        let inserted = upsert(&db, given).await.unwrap();

        let actual = get_by_slug(&db, inserted.slug).await.unwrap();

        let expected_profile_id = profile_id;
        let expected_name = name;
        let expected_slug = slug;
        let expected_description = description;

        assert_eq!(actual.profile_id, expected_profile_id);
        assert_eq!(actual.name, expected_name);
        assert_eq!(actual.slug, expected_slug);
        assert_eq!(actual.description, Some(expected_description));
    }

    #[sqlx::test]
    async fn test_get_by_username(db: Database) {
        // Create user to own profile.
        let username = "llavascript".to_string();
        let user = user::UserForUpsert::new(username.clone(), None);
        let user = user::upsert(&db, user).await.unwrap();

        // Create profile to own blog.
        let display_name = "Llava Script".to_string();
        let profile = profile::ProfileForUpsert::new(user.id.to_string(), display_name, None);
        let profile = profile::upsert(&db, profile).await.unwrap();

        let profile_id = Uuid::parse_str(&profile.id.unwrap()).unwrap();

        // Create several blogs
        {
            let name = "My Favorite JS Frameworks".to_string();
            let slug = "my-favorite-js-frameworks".to_string();
            let description = "A blog about my favorite JS frameworks".to_string();
            let given = BlogForUpsert::new(name.clone(), slug.clone(), profile_id)
                .with_description(description.clone());
            let _ = upsert(&db, given).await.unwrap();
        }
        {
            let name = "HTMX is the Future".to_string();
            let slug = "html-is-the-future".to_string();
            let description = "This is everything you need to know about HTMX.".to_string();
            let given = BlogForUpsert::new(name.clone(), slug.clone(), profile_id)
                .with_description(description.clone());
            let _ = upsert(&db, given).await.unwrap();
        }
        {
            let name = "JQuery Chronicles".to_string();
            let slug = "jquery-chronicles".to_string();
            let description = "Tales from the Heydays of JQuery.".to_string();
            let given = BlogForUpsert::new(name.clone(), slug.clone(), profile_id)
                .with_description(description.clone());
            let _ = upsert(&db, given).await.unwrap();
        }

        let actual = get_by_username(&db, username).await.unwrap();
        assert_eq!(actual.len(), 3);
    }
}

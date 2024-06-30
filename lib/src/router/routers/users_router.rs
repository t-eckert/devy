use crate::entities::User;
use crate::router::error::Result;
use crate::router::middleware::auth;
use crate::store::Store;
use crate::{db::user, router::Error};
use axum::{
    extract::{Path, State},
    middleware,
    routing::get,
    Json, Router,
};
use serde_json::Value;

pub struct UsersRouter;

impl UsersRouter {
    pub fn create(store: Store) -> Router<Store> {
        Router::new()
            .route("/users/:username", get(get_by_username))
            .route("/users/:username/github/repos", get(get_user_github_repos))
            .route(
                "/users/:username/github/devy",
                get(get_user_github_devy)
                    .layer(middleware::from_fn_with_state(store.clone(), auth)),
            )
            .with_state(store)
    }
}

/// `GET /users/:username`
///
/// Get a user by their username.
async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<User>> {
    Ok(Json(user::get_by_username(&store.db, &username).await?))
}

/// `GET /users/:username/github/repos`
///
/// Get a user's GitHub repositories.
async fn get_user_github_repos(Path(username): Path<String>) -> Result<Json<Value>> {
    match reqwest::Client::new()
        .get(&format!(
            "https://api.github.com/users/{}/repos?per_page=100&sort=updated",
            username
        ))
        .header("User-Agent", "devy")
        .header("Accept", "application/json")
        .send()
        .await
    {
        Ok(response) => {
            Ok(Json(response.json().await.map_err(|_| {
                Error::Generic("Request to GitHub Failed".to_string())
            })?))
        }
        Err(_) => Err(Error::Generic("Request to GitHub Failed".to_string())),
    }
}

/// `GET /users/:username/github/devy`
///
/// Get a user's GitHub Devy metadata.
async fn get_user_github_devy(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Value>> {
    let user_installations = store
        .github_client
        .fetch_user_installations()
        .await
        .map_err(|_| {
            Error::Generic("Unable to fetch GitHub application installations".to_string())
        })?;

    let has_devy_installed = user_installations.contains(&username);

    Ok(Json(
        serde_json::json!({"has_devy_installed": has_devy_installed}),
    ))
}

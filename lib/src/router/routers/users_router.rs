use crate::db::user;
use crate::entities::User;
use crate::router::error::Result;
use crate::store::Store;
use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

pub struct UsersRouter;

impl UsersRouter {
    pub fn create(store: Store) -> Router<Store> {
        Router::new()
            .route("/users/:username", get(get_by_username))
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
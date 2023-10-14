use crate::{entities::User, store::Store};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

pub async fn get_user_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<User>, StatusCode> {
    Ok(Json(
        User::get_by_username(&store.pool, username)
            .await
            .map_err(|_| StatusCode::NOT_FOUND)?,
    ))
}

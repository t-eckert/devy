use super::error::Result;
use crate::{entities::User, store::Store};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<User>> {
    Ok(Json(User::get_by_username(&store.pool, username).await?))
}

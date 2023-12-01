use super::error::Result;
use crate::{entities::Profile, store::Store};
use axum::{
    extract::{Path, State},
    Json,
};

pub async fn get_by_username(
    State(store): State<Store>,
    Path(username): Path<String>,
) -> Result<Json<Profile>> {
    Ok(Json(Profile::get_by_username(&store.pool, username).await?))
}

use crate::{
    entities::{Like, Post},
    store::Store,
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

pub async fn get_post_by_post_id(
    State(store): State<Store>,
    Path(post_id): Path<String>,
) -> Result<Json<Post>, StatusCode> {
    Ok(Json(
        Post::get_by_id(&store.pool, post_id)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

pub async fn post_like_to_post(
    headers: HeaderMap,
    State(store): State<Store>,
    Path(post_id): Path<String>,
) -> Result<Json<Like>, StatusCode> {
    dbg!(&headers);

    let token = headers
        .get("Authorization")
        .ok_or(StatusCode::BAD_REQUEST)?
        .to_str()
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .to_string();

    dbg!(&token);

    let profile_id = String::from("e2f0fa7e-4517-4ac8-bbc6-73067d3feed4"); // token.split_whitespace().collect::<Vec<&str>>()[1].to_string();

    println!("profile_id: {}", profile_id);

    Ok(Json(
        Like::new(profile_id, post_id)
            .upsert(&store.pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
    ))
}

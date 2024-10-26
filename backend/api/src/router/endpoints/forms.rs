use crate::router::{error::Result, middleware::auth};
use crate::store::Store;
use axum::{
    extract::{Json as ExtractJson, State},
    http::StatusCode,
    middleware,
    routing::post,
    Extension, Json, Router,
};
use lib::uploads::UploadRepository;
use lib::{
    forms::new_blog::{NewBlog, NewBlogResponse},
    token::Session,
};

/// Create a new router for forms.
pub fn router(store: Store) -> Router<Store> {
    Router::new()
        .route("/forms/new-blog", post(new_blog))
        .layer(middleware::from_fn_with_state(store.clone(), auth))
        .with_state(store)
}

// POST /forms/new-blog
async fn new_blog(
    Extension(session): Extension<Session>,
    State(store): State<Store>,
    ExtractJson(new_blog): ExtractJson<NewBlog>,
) -> Result<Json<NewBlogResponse>> {
    if session.user_id != new_blog.user_id {
        return Err(StatusCode::FORBIDDEN.into());
    }

    let res = new_blog.process(&store.db_conn).await?;

    let id = UploadRepository::insert(&store.db_conn, None, &res.repo.url).await?;

    let upload = UploadRepository::get(&store.db_conn, id).await?;
    store.uploader.upload(&store.db_conn, upload).await?;

    Ok(Json(res))
}

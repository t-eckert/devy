use crate::error::Result;
use axum::{
    extract::{Json as ExtractJson, State},
    routing::post,
    Json,
};
use forms::new_blog::{NewBlog, NewBlogResponse};
use store::Store;

pub struct FormsRouter;

/// /forms routes
impl FormsRouter {
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route("/forms/new-blog", post(new_blog))
            .with_state(store)
    }
}

/// POST /forms/new-blog
///
/// Creates a new blog.
async fn new_blog(
    State(store): State<Store>,
    ExtractJson(new_blog): ExtractJson<NewBlog>,
) -> Result<Json<NewBlogResponse>> {
    dbg!(&new_blog);
    Ok(Json(new_blog.process(&store.db).await?))
}

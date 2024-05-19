use crate::{
    forms::new_blog::{NewBlog, NewBlogResponse},
    router::{error::Result, middleware::auth},
    store::Store,
};
use axum::{
    extract::{Json as ExtractJson, State},
    middleware,
    routing::post,
    Json,
};

pub struct FormsRouter;

/// /forms routes
impl FormsRouter {
    pub fn create(store: Store) -> axum::Router<Store> {
        axum::Router::new()
            .route(
                "/forms/new-blog",
                post(new_blog).layer(middleware::from_fn_with_state(store.clone(), auth)),
            )
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
    Ok(Json(new_blog.process(&store.db).await?))
}

use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

use crate::db::DB;

use crate::entities::{Blog, BlogController, Post, PostController};

pub fn routes() -> Vec<Route> {
    routes![
        get_blog_by_blog_slug,
        get_post_by_blog_slug,
        get_post_by_blog_and_post_slug
    ]
}

#[post("/", format = "json", data = "<blog>")]
async fn upsert_blog(db: Connection<DB>, blog: Json<Blog>) -> Option<Json<Blog>> {
    Some(Json(BlogController::upsert(db, blog.into_inner()).await?))
}

#[get("/<blog_slug>")]
async fn get_blog_by_blog_slug(db: Connection<DB>, blog_slug: String) -> Option<Json<Blog>> {
    Some(Json(BlogController::get_by_slug(db, blog_slug).await?))
}

#[get("/<blog_slug>/posts")]
async fn get_post_by_blog_slug(db: Connection<DB>, blog_slug: String) -> Option<Json<Vec<Post>>> {
    Some(Json(PostController::get_by_blog_slug(db, blog_slug).await?))
}

#[get("/<blog_slug>/posts/<post_slug>")]
async fn get_post_by_blog_and_post_slug(
    db: Connection<DB>,
    blog_slug: String,
    post_slug: String,
) -> Option<Json<Post>> {
    Some(Json(
        PostController::get_by_blog_slug_and_post_slug(db, blog_slug, post_slug).await?,
    ))
}

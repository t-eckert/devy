use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

use crate::db::DB;

use crate::entities::{Blog, BlogController};

pub fn routes() -> Vec<Route> {
    routes![upsert_blog, get_blog_by_slug]
}

#[post("/", format = "json", data = "<blog>")]
async fn upsert_blog(db: Connection<DB>, blog: Json<Blog>) -> Option<Json<Blog>> {
    match BlogController::upsert(db, blog.into_inner()).await {
        Some(blog) => Some(Json(blog)),
        None => None,
    }
}

#[get("/<slug>")]
async fn get_blog_by_slug(db: Connection<DB>, slug: String) -> Option<Json<Blog>> {
    match BlogController::get_by_slug(db, slug).await {
        Some(blog) => Some(Json(blog)),
        None => None,
    }
}

// fn get_blog_by_blog_slug(db: Connection<DB>, blog_slug: String) -> Option<Json<Post>> {
//     Some(Json(Blog::get_by_slug(db, blog_slug)?))
// }

// Gets a post by its blog slug and post slug.
// #[get("/<blog_slug>/<post_slug>")]
// fn get_post_by_blog_and_post_slug(
//     db: Connection<DB>,
//     blog_slug: String,
//     post_slug: String,
// ) -> Option<Json<Post>> {
//     Some(Json(Post::get_by_blog_and_post_slug(
//         db, blog_slug, post_slug,
//     )?))
// }

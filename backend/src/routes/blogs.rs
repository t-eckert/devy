use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

/// Gets a post by its blog slug and post slug.
#[get("/blogs/<blog_slug>/<post_slug>")]
pub fn get_post_by_blog_and_post_slug(
    db: Connection<DB>,
    blog_slug: String,
    post_slug: String,
) -> Option<Json<Post>> {
    Some(Json(Post::get_by_blog_and_post_slug(
        db, blog_slug, post_slug,
    )?))
}

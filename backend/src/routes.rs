use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

/// Health check endpoint when the server is up and running.
#[get("/ready")]
pub fn ready() -> rocket::http::Status {
    rocket::http::Status::Ok
}

/// Gets a feed by its ID.
#[get("/feeds/<id>")]
pub fn get_feed_by_id(db: Connection<DB>, id: String) -> Option<Json<Feed>> {
    Some(Json::from(Feed::get_by_id(db, id)?))
}

/// Gets a user by their ID.
#[get("/users/<id>")]
pub fn get_user_by_id(db: Connection<DB>, id: String) -> Option<Json<User>> {
    Some(Json::from(User::get_by_id(db, id)?))
}

/// Gets a post by its ID.
#[get("/posts/<id>")]
pub fn get_post_by_id(db: Connection<DB>, id: &str) -> Option<Json<Post>> {
    Some(Json(Post::get_by_id(db, id)?))
}

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

/// Creates a like for a post.
#[post("/likes")]
pub fn post_like(db: Connection<DB>) -> Option<Json<Like>> {
    let like = Like::new(
        "246efba4-3b2e-4ebf-a5c0-80bb779627e0".to_string(),
        "ad5e2115-c095-4707-b784-15440be4a99b".to_string(),
    );
    Like::upsert(&like, db);
    Some(Json::from(like))
}

/// Delete a like for a post.
#[delete("/likes")]
pub fn delete_like(db: Connection<DB>) -> Option<Json<Like>> {
    let like = Like::new(
        "246efba4-3b2e-4ebf-a5c0-80bb779627e0".to_string(),
        "ad5e2115-c095-4707-b784-15440be4a99b".to_string(),
    );
    Like::delete(&like, db);
    Some(Json::from(like))
}

/// Catch all for 404 errors.
#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "resource not found"
    })
}

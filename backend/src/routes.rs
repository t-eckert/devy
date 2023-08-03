use rocket::serde::json::{json, Json, Value};
use sqlx::Connection;

use crate::models::{Feed, Post, User};

#[get("/ready")]
pub fn ready() -> rocket::http::Status {
    rocket::http::Status::Ok
}

#[get("/add")]
pub fn add(mut db: Connection<DB>) -> Value {
    let row = sqlx::query("SELECT 1 + 1").fetch_one(&mut *db).await?;

    json!({
        "status": "ok",
        "result": row.get::<i32, _>(0)
    })
}

#[get("/feeds/<id>")]
pub fn get_feed_by_id(id: String) -> Option<Json<Feed>> {
    Some(Json::from(Feed::get_by_id(id)?))
}

#[get("/users/<id>")]
pub fn get_user_by_id(id: &str) -> Json<User> {
    Json(User::new(
        id.to_string(),
        "02f4ca93-d856-4a16-8646-a50126eadd85".to_string(),
        "john.doe@gmail.com".to_string(),
    ))
}

#[get("/posts/<id>")]
pub fn get_post_by_id(id: &str) -> Option<Json<Post>> {
    Some(Json(Post::get_by_id(id)?))
}

#[get("/blogs/<blog_slug>/<post_slug>")]
pub fn get_post_by_blog_and_post_slug(blog_slug: String, post_slug: String) -> Option<Json<Post>> {
    Some(Json(Post::get_by_blog_and_post_slug(blog_slug, post_slug)?))
}

#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "resource not found"
    })
}

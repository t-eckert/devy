pub mod blogs;
pub mod bookmarks;
pub mod feeds;
pub mod likes;
pub mod posts;
pub mod uploads;
pub mod users;
pub mod webhooks;

use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

/// Health check endpoint when the server is up and running.
#[get("/ready")]
pub fn ready() -> rocket::http::Status {
    rocket::http::Status::Ok
}

/// Catch all for 404 errors.
#[catch(404)]
pub fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "resource not found"
    })
}

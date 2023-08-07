use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_id]
}

/// Gets a feed by its ID.
#[get("/<feed_id>")]
pub fn get_by_id(db: Connection<DB>, feed_id: String) -> Option<Json<Feed>> {
    Some(Json::from(Feed::get_by_id(db, feed_id)?))
}

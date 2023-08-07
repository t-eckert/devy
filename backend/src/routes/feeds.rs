use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::models::{Feed, Like, Post, User};

/// Gets a feed by its ID.
#[get("/feeds/<id>")]
pub fn get_by_id(db: Connection<DB>, id: String) -> Option<Json<Feed>> {
    Some(Json::from(Feed::get_by_id(db, id)?))
}

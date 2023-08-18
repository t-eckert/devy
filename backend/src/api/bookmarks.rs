use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::Bookmark;

pub fn routes() -> Vec<rocket::Route> {
    routes![upsert, delete]
}

/// Creates a bookmark for a post.
#[post("/", format = "json", data = "<bookmark>")]
fn upsert(db: Connection<DB>, bookmark: Json<Bookmark>) -> Option<Json<Bookmark>> {
    unimplemented!()
}

/// Deletes a bookmark for a post.
#[delete("/", format = "json", data = "<bookmark>")]
fn delete(db: Connection<DB>, bookmark: Json<Bookmark>) -> Option<Json<Bookmark>> {
    unimplemented!()
}

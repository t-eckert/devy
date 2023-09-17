use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::{Upload, UploadController};

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_id]
}

/// Gets an upload by its ID.
#[get("/<upload_id>", format = "json")]
fn get_by_id(db: Connection<DB>, upload_id: String) -> Option<Json<Upload>> {
    unimplemented!()
}

use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::Feed;

pub fn routes() -> Vec<rocket::Route> {
    routes![get_by_id]
}

/// Gets a feed by its ID.
#[get("/<feed_id>")]
fn get_by_id(db: Connection<DB>, feed_id: String) -> Option<Json<Feed>> {
    unimplemented!()
}

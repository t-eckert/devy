use rocket::serde::json::Json;

use super::Feed;

#[get("/<id>")]
pub fn get_feed_by_id(id: String) -> Json<Feed> {
    Json::from(Feed::get_by_id(id))
}

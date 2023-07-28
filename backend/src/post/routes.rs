use rocket::serde::json::Json;

use super::fixtures::get_post_0001;
use super::Post;

#[get("/<id>")]
pub fn get_post_by_id(id: &str) -> Json<Post> {
    Json(get_post_0001())
}

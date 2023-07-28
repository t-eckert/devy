use rocket::serde::json::Json;

use super::Post;

#[get("/<id>")]
pub fn get_post_by_id(id: &str) -> Json<Post> {
    Json::from(Post::get_by_id(id))
}

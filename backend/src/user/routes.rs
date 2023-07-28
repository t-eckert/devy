use rocket::serde::json::Json;

use super::User;

#[get("/<id>")]
pub fn get_user_by_id(id: &str) -> Json<User> {
    Json(User {
        id: id.to_string(),
        name: "John".to_string(),
        email: "john.doe@gmail.com".to_string(),
    })
}

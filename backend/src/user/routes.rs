use rocket::serde::json::Json;

use super::User;

#[get("/<id>")]
pub fn get_user_by_id(id: &str) -> Json<User> {
    Json(User::new(
        id.to_string(),
        "02f4ca93-d856-4a16-8646-a50126eadd85".to_string(),
        "john.doe@gmail.com".to_string(),
    ))
}

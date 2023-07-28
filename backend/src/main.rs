#[macro_use]
extern crate rocket;

mod post;
mod user;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![ready])
        .mount("/posts", routes![post::routes::get_post_by_id])
        .mount("/users", routes![user::routes::get_user_by_id])
}

#[get("/ready")]
fn ready() -> rocket::http::Status {
    rocket::http::Status::Ok
}

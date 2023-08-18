use rocket::response::Redirect;

pub fn routes() -> Vec<rocket::Route> {
    routes![login]
}

#[get("/login")]
async fn login() -> Redirect {
    Redirect::to(uri!("http:localhost:3000"))
}

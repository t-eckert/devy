#[get("/new")]
pub fn get_new_feed() -> &'static str {
    r#"{"hello": "world"}"#
}

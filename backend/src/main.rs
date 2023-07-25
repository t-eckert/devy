use poem::{get, handler, listener::TcpListener, web::Path, IntoResponse, Route, Server};

#[handler]
fn feed_new() -> String {
    String::from(r#"{"id":"new","slug":"new","name":"New","filters":[],"posts":[]}"#)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/feeds/new", get(feed_new));

    Server::new(TcpListener::bind("127.0.0.1:3001"))
        .run(app)
        .await
}

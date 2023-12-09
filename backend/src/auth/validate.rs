use axum::{extract::Request, http, middleware::Next, response::Response};
use http::HeaderMap;

pub async fn is_authenticated(headers: HeaderMap, request: Request, next: Next) -> Response {
    let auth_header = match headers.get("Authorization") {
        Some(header) => header,
        None => {
            return http::Response::builder()
                .status(http::StatusCode::UNAUTHORIZED)
                .body("Unauthorized".into())
                .unwrap()
        }
    };
    dbg!(&auth_header);

    if auth_header != "Bearer 123" {
        return http::Response::builder()
            .status(http::StatusCode::UNAUTHORIZED)
            .body("Unauthorized".into())
            .unwrap();
    }

    let response = next.run(request).await;

    response
}

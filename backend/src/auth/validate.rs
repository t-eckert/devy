use axum::{
    extract::Request, http, http::HeaderMap, http::HeaderValue, middleware::Next,
    response::Response,
};

pub async fn is_authenticated(headers: HeaderMap, request: Request, next: Next) -> Response {
    let auth_header = match get_header(headers) {
        Ok(header) => header,
        Err(response) => return response,
    };

    let _ = match get_session(auth_header) {
        Ok(_) => (),
        Err(response) => return response,
    };

    let response = next.run(request).await;

    response
}

fn get_header(headers: HeaderMap) -> Result<HeaderValue, Response> {
    match headers.get("Authorization") {
        Some(header) => Ok(header.clone()),
        None => {
            return Err(http::Response::builder()
                .status(http::StatusCode::UNAUTHORIZED)
                .body("Unauthorized".into())
                .unwrap())
        }
    }
}

fn get_session(token: HeaderValue) -> Result<(), Response> {
    if token != "Bearer 123" {
        return Err(http::Response::builder()
            .status(http::StatusCode::UNAUTHORIZED)
            .body("Unauthorized".into())
            .unwrap());
    }

    Ok(())
}

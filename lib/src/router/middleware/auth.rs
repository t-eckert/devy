use crate::{
    router::error::{Error, Result},
    store::Store,
};
use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::Response,
};
use http::StatusCode;

const AUTHORIZATION_HEADER: &str = "Authorization";
const BEARER_PREFIX: &str = "Bearer";

/// Middleware to authenticate requests.
pub async fn auth(
    headers: HeaderMap,
    State(store): State<Store>,
    request: Request,
    next: Next,
) -> Result<Response> {
    match get_token(&headers) {
        Ok(token) => {
            // Verify the token
            let session = store
                .auth_client
                .validate_token(token)
                .await
                .map_err(|_| Error::Generic("Invalid token".to_string()))?;
            dbg!(&session);
            Ok(next.run(request).await)
        }
        Err(e) => Err(e),
    }
}

fn get_token(headers: &HeaderMap) -> Result<&str> {
    let auth_header = headers
        .get(AUTHORIZATION_HEADER)
        .and_then(|value| value.to_str().ok())
        .ok_or(Error::StatusCode(StatusCode::BAD_REQUEST))?;

    match auth_header.split_once(' ') {
        Some((BEARER_PREFIX, token)) => Ok(token),
        _ => Err(Error::StatusCode(StatusCode::BAD_REQUEST)),
    }
}

#[cfg(test)]
mod tests {
    use http::HeaderValue;

    use super::*;

    #[test]
    pub fn test_get_token() {
        let mut headers = HeaderMap::new();
        headers.insert(
            "Authorization",
            HeaderValue::from_str("Bearer 1234").unwrap(),
        );

        assert_eq!(get_token(&headers).unwrap(), "1234");
    }
}

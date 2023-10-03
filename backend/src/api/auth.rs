use crate::auth;
use axum::{
    extract::{Query, State},
    response::Redirect,
};
use std::collections::HashMap;

pub async fn login(State(auth_client): State<auth::Client>) -> Redirect {
    Redirect::permanent(&auth_client.redirect_uri())
}

/// callback is the endpoint that GitHub redirects to after a successful login
/// It exchanges the code for a token and then fetches the user from GitHub.
/// If the user doesn't exist in the database, it creates a new user.
/// It then creates a session for the user and returns a JWT.
pub async fn callback(
    State(auth_client): State<auth::Client>,
    Query(params): Query<HashMap<String, String>>,
) -> Redirect {
    let code = params.get("code").expect("No code in callback");
    dbg!(code);

    Redirect::permanent(&auth_client.post_auth_redirect_uri())
}

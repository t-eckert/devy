use crate::auth;
use axum::{
    extract::{Query, State},
    response::Redirect,
};

pub async fn login(State(auth_client): State<auth::Client>) -> Redirect {
    Redirect::permanent(&auth_client.redirect_uri())
}

/// callback is the endpoint that GitHub redirects to after a successful login
/// It exchanges the code for a token and then fetches the user from GitHub.
/// If the user doesn't exist in the database, it creates a new user.
/// It then creates a session for the user and returns a JWT.
pub async fn callback(
    State(auth_client): State<auth::Client>,
    Query(code): Query<Option<String>>,
) -> Redirect {
    let code = code.expect("No code provided");
    dbg!(code);

    Redirect::permanent(&auth_client.post_auth_redirect_uri())
}

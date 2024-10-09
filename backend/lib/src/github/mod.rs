mod auth_provider;
mod client;
mod error;
mod github_user;

pub use client::Client;
pub use error::{Error, Result};
pub use github_user::GitHubUser;

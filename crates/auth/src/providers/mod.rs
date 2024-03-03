mod github;
mod local;

pub use github::GitHubProvider;
pub use local::LocalProvider;

#[derive(Clone)]
pub enum Provider {
    GitHub,
    Local,
}

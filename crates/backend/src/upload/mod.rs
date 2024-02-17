mod error;
mod git;
mod uploader;

pub use error::Error;
pub use git::Git;
pub use uploader::Uploader;

use std::env;

pub fn setup_uploader() -> Uploader {
    let git_path = env::var("GIT_PATH").expect("GIT_PATH not set");
    let git = Git::new(git_path).expect("Unable to create git client");

    Uploader::new(git)
}

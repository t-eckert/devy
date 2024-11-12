mod error;
mod git;

pub use error::Error;
pub use git::{find_git_or_panic, Git};

use error::Result;

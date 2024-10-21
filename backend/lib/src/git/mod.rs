mod diff;
mod error;
mod git;

pub use diff::Diff;
pub use error::Error;
pub use git::{find_git_or_panic, Git};

use error::Result;

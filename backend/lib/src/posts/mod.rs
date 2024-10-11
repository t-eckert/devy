mod bookmark;
mod entry;
mod error;
mod like;
mod post;

use error::Result;

pub use error::Error;

pub use bookmark::{Bookmark, BookmarkRepository};
pub use entry::{Entry, EntryRepository};
pub use like::{Like, LikeRepository};
pub use post::{Post, PostRepository};

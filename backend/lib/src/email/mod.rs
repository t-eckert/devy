mod email;
mod send;
mod queries;
mod error;

pub use email::Email;
pub use send::send;
pub use error::{Error,Result};

use super::Result;
use crate::{db::blog, store::Store};
use lib::posts::Like;

pub struct LikesController;

impl LikesController {
    pub async fn get_by_username(store: &Store, username: &String) -> Result<Vec<Like>> {
        Ok(vec![])
    }
}

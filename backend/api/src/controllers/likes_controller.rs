use super::Result;
use crate::store::Store;
use lib::posts::Like;

pub struct LikesController;

impl LikesController {
    pub async fn get_by_username(store: &Store, username: &String) -> Result<Vec<Like>> {
        Ok(vec![])
    }
}

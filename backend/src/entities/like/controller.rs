use rocket_db_pools::Connection;

use crate::db::DB;
use crate::entities::Like;

pub struct LikeController {}

impl LikeController {
    pub fn upsert(mut db: Connection<DB>, like: Like) -> Option<Like> {
        unimplemented!()
    }

    pub fn delete(mut db: Connection<DB>, like: Like) -> Option<Like> {
        unimplemented!()
    }
}

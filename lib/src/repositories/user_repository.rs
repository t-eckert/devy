#[allow(dead_code, unused_variables)]
use uuid::Uuid;

use crate::db;
use crate::user::User;

pub struct UserRepository;

impl UserRepository {
    pub async fn insert(db_conn: &db::Conn, user: &User) -> db::Result<db::Id> {
        unimplemented!()
    }

    pub async fn update(db_conn: &db::Conn, user: &User) -> db::Result<db::Id> {
        unimplemented!()
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Option<User>> {
        unimplemented!()
    }

    pub async fn get_by_username(db_conn: &db::Conn, username: &str) -> db::Result<Option<User>> {
        unimplemented!()
    }

    pub async fn get_by_github_username(
        db_conn: &db::Conn,
        github_username: &str,
    ) -> db::Result<Option<User>> {
        unimplemented!()
    }

    pub async fn get_all(db_conn: &db::Conn) -> db::Result<Vec<User>> {
        unimplemented!()
    }

    pub async fn count(db_conn: &db::Conn) -> db::Result<i64> {
        unimplemented!()
    }

    pub async fn delete(db_conn: &db::Conn, id: Uuid) -> db::Result<()> {
        unimplemented!()
    }
}

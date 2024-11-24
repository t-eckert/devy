use crate::{db, identity::Identity};
use uuid::Uuid;

pub struct IdentityRepository;

impl IdentityRepository {
    pub async fn insert(
        db_conn: &db::Conn,
        username: &str,
        email: Option<&str>,
    ) -> db::Result<db::Id> {
        unimplemented!()
    }

    pub async fn update(db_conn: &db::Conn, identity: Identity) -> db::Result<db::Id> {
        unimplemented!()
    }

    pub async fn get(db_conn: &db::Conn, id: Uuid) -> db::Result<Option<Identity>> {
        unimplemented!()
    }

    pub async fn get_by_username(
        db_conn: &db::Conn,
        username: &str,
    ) -> db::Result<Option<Identity>> {
        unimplemented!()
    }

    pub async fn get_by_email(db_conn: &db::Conn, email: &str) -> db::Result<Option<Identity>> {
        unimplemented!()
    }

    pub async fn get_by_github_username(
        db_conn: &db::Conn,
        github_username: &str,
    ) -> db::Result<Option<Identity>> {
        unimplemented!()
    }

    pub async fn get_all(db_conn: &db::Conn) -> db::Result<Vec<Identity>> {
        unimplemented!()
    }

    pub async fn count(db_conn: &db::Conn) -> db::Result<i64> {
        unimplemented!()
    }

    pub async fn delete(db_conn: &db::Conn, id: Uuid) -> db::Result<()> {
        unimplemented!()
    }
}

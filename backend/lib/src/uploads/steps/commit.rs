use crate::{
    db::Conn,
    repositories::RepoRepository,
    uploads::{Error, Result, Status, Upload},
};

pub async fn commit(db_conn: &Conn, mut upload: Upload) -> Result<Upload> {
    dbg!("commit");

    upload.set_status(Status::COMMITTED);
    Ok(upload)
}

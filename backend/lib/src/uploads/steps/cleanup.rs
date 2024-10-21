use crate::{
    db::Conn,
    repositories::RepoRepository,
    uploads::{Error, Result, Status, Upload},
};

pub async fn cleanup(db_conn: &Conn, mut upload: Upload) -> Result<Upload> {
    dbg!("cleanup");

    upload.set_status(Status::DONE);
    Ok(upload)
}

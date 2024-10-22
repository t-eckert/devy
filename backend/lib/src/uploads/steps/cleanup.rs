use crate::{
    db::Conn,
    repositories::RepoRepository,
    uploads::{Error, Result, Status, Upload},
};

/// The cleanup step removes the temporary directory created for the upload.
pub async fn cleanup(db_conn: &Conn, mut upload: Upload) -> Result<Upload> {
    dbg!("cleanup");

    match std::fs::remove_dir_all(&upload.dir()).map_err(|_| Error::CleanupFailure) {
        Ok(_) => {}
        Err(err) => {
            upload.set_status(Status::FAILED);
            upload.append_log(&format!("ERROR: Could not remove directory: {}", err));
            return Ok(upload);
        }
    }

    upload.set_status(Status::DONE);
    Ok(upload)
}

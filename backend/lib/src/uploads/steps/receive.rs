use crate::{
    db::Conn,
    repositories::RepoRepository,
    uploads::{Error, Result, Status, Upload, UploadRepository},
};

/// The receive step marks the upload as received.
/// It checks for a previous upload and sets it on the upload as its predecessor.
pub async fn receive(db_conn: &Conn, mut upload: Upload) -> Result<Upload> {
    tracing::info!("Receiving upload {}", upload.id);

    dbg!("receive");

    if upload.status != Status::VERIFIED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not verified");
        return Ok(upload);
    }

    // Set the previous upload if it exists.
    match UploadRepository::get_previous(db_conn, &upload).await? {
        Some(previous_upload) => {
            upload.previous_upload_id = Some(previous_upload.id);
        }
        None => {
            upload.previous_upload_id = None;
        }
    }

    upload.set_status(Status::RECEIVED);
    upload.append_log("INFO: Upload received by uploader");
    tracing::info!("Upload received {}", upload.id);
    Ok(upload)
}

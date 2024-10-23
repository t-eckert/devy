use crate::{
    db::Conn,
    repositories::RepoRepository,
    uploads::{Status, Upload, UploadRepository},
};

/// The receive step marks the upload as received.
/// It checks for a previous upload and sets it on the upload as its predecessor.
pub async fn receive(db_conn: &Conn, mut upload: Upload) -> Upload {
    tracing::info!("Receiving upload {}", upload.id);

    if upload.status != Status::VERIFIED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not verified");
        return upload;
    }

    let previous = match UploadRepository::get_previous(db_conn, &upload).await {
        Ok(prev) => prev,
        Err(err) => {
            upload.set_status(Status::FAILED);
            upload.append_log(&format!("ERROR: Could not get previous upload: {}", err));
            return upload;
        }
    };

    upload.previous_upload_id = previous.map(|prev| prev.id);
    if upload.has_previous() {
        upload.append_log(&format!(
            "INFO: Previous upload: {}",
            upload.previous_upload_id.unwrap_or_default(),
        ));
    } else {
        upload.append_log("INFO: No previous upload");
    }

    upload.set_status(Status::RECEIVED);
    upload.append_log("INFO: Upload received by uploader");
    tracing::info!("Upload received {}", upload.id);

    upload
}

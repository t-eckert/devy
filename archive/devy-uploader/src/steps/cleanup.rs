use lib::{
    db,
    uploads::{Status, Upload},
};

/// The cleanup step removes the temporary directory created for the upload.
pub async fn cleanup(_: &db::Conn, mut upload: Upload) -> Upload {
    tracing::info!("Cleaning up upload {}", upload.id);

    match std::fs::remove_dir_all(&upload.dir()) {
        Ok(_) => {}
        Err(err) => {
            upload.set_status(Status::FAILED);
            upload.append_log(&format!("ERROR: Could not remove directory: {}", err));
            return upload;
        }
    }

    upload.set_status(Status::DONE);
    upload.append_log("INFO: Upload cleaned up");
    tracing::info!("Upload cleaned up {}", upload.id);

    upload
}

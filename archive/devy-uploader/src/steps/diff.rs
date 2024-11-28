use lib::{
    db,
    git::Git,
    repositories::UploadRepository,
    uploads::{Status, Upload},
};

/// Diff compares the upload to the previous upload to identify changes.
pub async fn diff(db_conn: &db::Conn, mut upload: Upload, git: &Git) -> Upload {
    tracing::info!("Diffing upload {}", upload.id);

    if upload.status != Status::CLONED {
        upload.set_status(Status::REJECTED);
        upload.append_log("ERROR: Upload is not cloned");
        return upload;
    }

    // If the current SHA is "HEAD", then we need to get the actual SHA.
    // This is because the previous upload will have the actual SHA.
    if upload.sha == "HEAD" {
        upload.sha = match git.sha(&upload.dir()) {
            Ok(sha) => sha,
            Err(err) => {
                upload.set_status(Status::FAILED);
                upload.append_log(&format!("ERROR: Could not get HEAD sha: {}", err));
                return upload;
            }
        };
    }

    // Get the sha of the previous upload or the empty tree sha if no previous upload.
    // This is used to get the diff between the previous upload and the current upload.
    let from = match upload.previous_upload_id {
        Some(previous_upload_id) => {
            let previous = match UploadRepository::get(db_conn, previous_upload_id).await {
                Ok(prev) => prev,
                Err(err) => {
                    upload.set_status(Status::FAILED);
                    upload.append_log(&format!("ERROR: Could not get previous upload: {}", err));
                    return upload;
                }
            };
            previous.sha
        }
        None => git.empty_tree_sha(),
    };

    // Get the diff between the previous upload and the current upload.
    let diff = match git.diff(&upload.dir(), &upload.sha, &from) {
        Ok(diff) => diff,
        Err(err) => {
            upload.set_status(Status::FAILED);
            upload.append_log(&format!("ERROR: Could not get diff: {}", err));
            return upload;
        }
    };

    upload.set_diff(&diff);

    upload.set_status(Status::DIFFED);
    upload.append_log("INFO: Upload diffed");
    tracing::info!("Upload diffed {}", upload.id);

    upload
}

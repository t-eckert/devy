use super::Result;
use crate::store::Store;
use lib::uploader::{Upload, UploadRepository, Uploader};
use serde::{Deserialize, Serialize};

pub struct UploadsController;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NewUpload {
    pub repo: String,
}

impl UploadsController {
    pub async fn create_new_upload(store: &Store, new_upload: NewUpload) -> Result<Upload> {
        unimplemented!("create_new_upload")
    }
}

use super::Result;
use crate::store::Store;
use lib::uploads::Upload;
use serde::{Deserialize, Serialize};

pub struct UploadsController;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUpload {
    pub repo: String,
}

impl UploadsController {
    pub async fn get_uploads(_store: &Store) -> Result<Vec<Upload>> {
        unimplemented!("get_uploads")
    }

    #[allow(dead_code)]
    pub async fn create_new_upload(_store: &Store, _new_upload: NewUpload) -> Result<Upload> {
        unimplemented!("create_new_upload")
    }
}

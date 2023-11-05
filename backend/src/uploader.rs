use crate::entities::Upload;
use sqlx::PgPool;

#[derive(Clone)]
pub struct Uploader {}

impl Uploader {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn upload(self, upload: Upload, pool: &PgPool) -> Result<Upload, String> {
        println!("Upload initiated");

        Ok(upload)
    }
}

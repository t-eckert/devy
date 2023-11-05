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

        let upload_received = upload.set_status(pool, "received".to_string()).await;

        upload_received.map_err(|_| "AAAAAAHHHHH!!".to_string())
    }
}

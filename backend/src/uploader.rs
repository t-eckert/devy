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

        // I know I can nicely chain this later...

        let upload_logged_to = upload_received
            .map_err(|_| "AAAAAAHHHHH!!".to_string())?
            .log(pool, "INFO: Upload received by uploader.".to_string())
            .await;

        upload_logged_to.map_err(|_| "AAAAAAHHHHH!!".to_string())
    }
}

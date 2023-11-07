use crate::{entities::Upload, git::Git};
use sqlx::PgPool;

#[derive(Clone)]
pub struct Uploader {
    git: Git,
}

impl Uploader {
    pub fn new(git: Git) -> Self {
        Self { git }
    }

    pub async fn upload(self, upload: Upload, pool: &PgPool) -> Result<Upload, String> {
        println!("Upload initiated");

        let upload_received = upload.set_status(pool, "received".to_string()).await;

        // I know I can nicely chain this later...

        let upload_logged_to = upload_received
            .map_err(|_| "AAAAAAHHHHH!!".to_string())?
            .log(pool, "INFO: Upload received by uploader.".to_string())
            .await
            .map_err(|_| "AAAAAAHHHHH!!".to_string())?;

        self.git
            .clone_repo(&upload_logged_to.clone().repo.unwrap().as_str())
            .map_err(|_| "AAAAAAHHHHH!!".to_string())?;

        Ok(upload_logged_to)
    }
}

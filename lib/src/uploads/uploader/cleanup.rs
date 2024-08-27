use super::error::{Error, Result};
use crate::entities::Upload;

pub fn cleanup(upload: &mut Upload) -> Result<()> {
    tracing::info!("Cleaning up...");

    // Delete the temporary directory
    std::fs::remove_dir_all(upload.dir())
        .map_err(|_| Error::DependencyError("Unable to remove dir".to_string()))?;

    Ok(())
}

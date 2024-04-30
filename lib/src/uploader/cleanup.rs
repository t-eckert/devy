use super::error::Result;
use crate::entities::Upload;

pub fn cleanup(upload: &mut Upload) -> Result<()> {
    tracing::info!("Cleaning up...");

    Ok(())
}

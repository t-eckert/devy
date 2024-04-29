use entities::Upload;
use monitoring::tracing;

use crate::error::Result;

pub fn cleanup(upload: &mut Upload) -> Result<()> {
    tracing::info!("Cleaning up...");

    Ok(())
}

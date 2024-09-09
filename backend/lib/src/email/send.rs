use super::{Email, Result};

pub async fn send(email: Email) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send() {
    }
}

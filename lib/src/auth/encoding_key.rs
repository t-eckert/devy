use rand::{distributions::Alphanumeric, Rng};

const KEY_SIZE: usize = 64;

/// Generates a random encoding key of 64 bytes.
pub fn generate_encoding_key() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(KEY_SIZE)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_encoding_key() {
        let key = generate_encoding_key();
        assert_eq!(key.len(), KEY_SIZE);
    }
}

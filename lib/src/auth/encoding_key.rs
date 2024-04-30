use rand::{distributions::Alphanumeric, Rng};

const KEY_SIZE: usize = 32;

/// Generates a random encoding key of 32 bytes.
pub fn generate_encoding_key() -> String {
    let rng = rand::thread_rng();
    let key: String = rng
        .sample_iter(&Alphanumeric)
        .take(KEY_SIZE)
        .map(char::from)
        .collect();

    key
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

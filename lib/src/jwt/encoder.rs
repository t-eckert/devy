use super::{Error, Result};
use jsonwebtoken::EncodingKey;
use openssl::rsa::Rsa;

#[derive(Clone)]
pub struct Encoder {
    encoding_key: EncodingKey,
}

impl Encoder {
    /// Creates a new encoder.
    pub fn new(pem: &[u8]) -> Result<Self> {
        Ok(Self {
            encoding_key: EncodingKey::from_rsa_pem(pem)?,
        })
    }
    /// Encodes the given subject into a JWT.
    pub fn encode(&self) -> Result<String> {
        Ok("".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let pk = Rsa::generate(2048).unwrap();
        let pem = pk.private_key_to_pem().unwrap();

        let encoder = Encoder::new(pem.as_slice()).unwrap();
    }
}

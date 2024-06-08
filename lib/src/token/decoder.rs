use super::{jwt, Result, Session};
use jsonwebtoken::DecodingKey;

#[derive(Clone)]
pub struct Decoder {
    decoding_key: DecodingKey,
}

impl Decoder {
    /// Creates a new decoder using the given PEM-encoded RSA public key.
    pub fn new(public_pem: &[u8]) -> Result<Self> {
        Ok(Self {
            decoding_key: DecodingKey::from_rsa_pem(public_pem)?,
        })
    }

    /// Decodes the given token into a session.
    pub fn decode(&self, token: &str) -> Result<Session> {
        Ok(serde_json::from_value(jwt::decode(
            token,
            &self.decoding_key,
        )?)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Encoder;

    #[test]
    fn test_decoding_from_encoder() {
        let public_pem = include_bytes!("test_public_key.pem");
        let private_pem = include_bytes!("test_private_key.pem");

        let encoder = Encoder::new(public_pem, private_pem).unwrap();
        let decoder = Decoder::new(public_pem).unwrap();

        let session = Session::new();

        let token = encoder.encode(session.clone()).unwrap();
        let decoded_session = decoder.decode(&token).unwrap();

        assert_eq!(session, decoded_session);
    }
}

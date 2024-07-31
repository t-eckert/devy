use super::{jwt, session::Session, Error, Result};
use jsonwebtoken::{DecodingKey, EncodingKey};
use openssl::rsa::Rsa;

#[derive(Clone)]
pub struct Encoder {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Encoder {
    /// Creates a new encoder using the given PEM-encoded RSA private key.
    pub fn new(public_pem: &[u8], private_pem: &[u8]) -> Result<Self> {
        Ok(Self {
            encoding_key: EncodingKey::from_rsa_pem(private_pem)?,
            decoding_key: DecodingKey::from_rsa_pem(public_pem)?,
        })
    }

    /// Encodes the given session into a JWT.
    pub fn encode(&self, session: Session) -> Result<String> {
        jwt::encode(
            "user".to_string(),
            serde_json::to_value(session)?,
            &self.encoding_key,
        )
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

    #[test]
    fn test_encoding_and_decoding() {
        let public_pem = include_bytes!("test_public_key.pem");
        let private_pem = include_bytes!("test_private_key.pem");

        let encoder = Encoder::new(public_pem, private_pem).unwrap();

        let session = Session::new(
            uuid::Uuid::new_v4(),
            "username".to_string(),
            "role".to_string(),
            "status".to_string(),
            Some("display_name".to_string()),
            Some("avatar_url".to_string()),
        );

        let token = encoder.encode(session.clone()).unwrap();
        let decoded_session = encoder.decode(&token).unwrap();

        assert_eq!(session, decoded_session);
    }
}

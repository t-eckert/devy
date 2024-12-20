use core::fmt;

use super::{jwt, session::Session, Result};
use jsonwebtoken::{DecodingKey, EncodingKey};

/// An encoder for JWT tokens. Also capable of decoding tokens.
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

impl fmt::Debug for Encoder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Encoder").finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user;

    #[test]
    fn test_encoding_and_decoding() {
        let public_pem = include_bytes!("test_public_key.pem");
        let private_pem = include_bytes!("test_private_key.pem");

        let encoder = Encoder::new(public_pem, private_pem).unwrap();

        let session = Session::new(
            uuid::Uuid::new_v4(),
            uuid::Uuid::new_v4(),
            "username".to_string(),
            user::Role::User,
            user::Status::Active,
            Some("display_name".to_string()),
            Some("avatar_url".to_string()),
        );

        let token = encoder.encode(session.clone()).unwrap();
        let decoded_session = encoder.decode(&token).unwrap();

        assert_eq!(session, decoded_session);
    }
}

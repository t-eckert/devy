use derive_more::From;
use jsonwebtoken;
use openssl::rsa::Rsa;
use serde::{Deserialize, Serialize};
use serde_json::Value;

// How long the token is valid for in seconds.
const LIFETIME: u64 = 3600;

/// JSON Web Token (JWT) encoder/decoder
#[derive(Debug, Clone)]
pub struct JWT {
    /// The public key used to validate JWTs.
    pub public_key: String,

    private_key: String,
}

impl JWT {
    /// Create a new JWT encoder/decoder with the given encoding key.
    pub fn new(private_key: String) -> Result<Self> {
        let public_key = String::from_utf8(
            Rsa::private_key_from_pem(private_key.as_bytes())?.public_key_to_pem()?,
        )
        .map_err(|err| Error::JWTError(err.to_string()))?;

        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// Encode a set of claims into a JWT.
    pub fn encode(&self, subject: Subject, value: Value) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &self.header(),
            &Claims::new(subject, value),
            &jsonwebtoken::EncodingKey::from_rsa_pem(self.private_key.as_bytes())?,
        )?)
    }

    /// Decode a JWT into its subject and body.
    pub fn decode(&self, token: &str) -> Result<(Subject, Value)> {
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.private_key.as_ref()),
            &jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::RS256),
        )?
        .claims;

        Ok((claims.sub, claims.body))
    }

    fn header(&self) -> jsonwebtoken::Header {
        jsonwebtoken::Header::new(jsonwebtoken::Algorithm::RS256)
    }
}

/// A result type for JWT processes.
pub type Result<T> = core::result::Result<T, Error>;

/// Errors that can occur during JWT processes.
#[derive(Debug, From)]
pub enum Error {
    JWTError(String),

    #[from]
    OpenSSLError(openssl::error::ErrorStack),

    #[from]
    SerdeJson(serde_json::Error),

    #[from]
    JsonWebTokenError(jsonwebtoken::errors::Error),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

/// The subject of a JWT.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Subject {
    AuthToken,
}

/// The internal claims structure for the JWT.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Claims {
    sub: Subject,
    body: Value,
    exp: u64,
    iat: u64,
    nbf: u64,
}

impl Claims {
    fn new(sub: Subject, body: Value) -> Self {
        Self {
            sub,
            body,
            exp: jsonwebtoken::get_current_timestamp() + LIFETIME,
            iat: jsonwebtoken::get_current_timestamp(),
            nbf: jsonwebtoken::get_current_timestamp(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_jwt_has_proper_public_key() {
        let key_pair = Rsa::generate(2084).unwrap();

        let private_key = String::from_utf8(key_pair.private_key_to_pem().unwrap()).unwrap();

        let expected_public_key = String::from_utf8(key_pair.public_key_to_pem().unwrap()).unwrap();

        let jwt = JWT::new(private_key).unwrap();

        assert_eq!(jwt.public_key, expected_public_key);
    }

    #[test]
    fn test_jwt_with_valid_claim() {
        let private_key =
            String::from_utf8(Rsa::generate(2084).unwrap().private_key_to_pem().unwrap()).unwrap();

        let subject = Subject::AuthToken;

        let jwt = JWT::new(private_key).unwrap();
        let value = serde_json::json!({"test": "value"});

        let token = jwt.encode(Subject::AuthToken, value.clone()).unwrap();

        let actual = jwt.decode(&token).unwrap();

        assert_eq!(actual.0, subject);
        assert_eq!(actual.1, value);
    }
}

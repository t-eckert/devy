use super::error::Result;
use jsonwebtoken;
use serde::{Deserialize, Serialize};
use serde_json::Value;

const LIFETIME: u64 = 3600;

/// JSON Web Token (JWT) encoder/decoder
#[derive(Debug, Clone)]
pub struct JWT {
    encoding_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    sub: String,
    value: Value,
    exp: u64,
    iat: u64,
    nbf: u64,
}

impl Claims {
    pub fn new(sub: &str, value: Value) -> Self {
        Self {
            sub: sub.to_string(),
            value,
            exp: jsonwebtoken::get_current_timestamp() + LIFETIME,
            iat: jsonwebtoken::get_current_timestamp(),
            nbf: jsonwebtoken::get_current_timestamp(),
        }
    }
}

impl JWT {
    /// Create a new JWT encoder/decoder with the given encoding key.
    pub fn new(encoding_key: String) -> Self {
        Self { encoding_key }
    }

    /// Encode a set of claims into a JWT.
    pub fn encode(&self, sub: &str, value: Value) -> Result<String> {
        Ok(jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &Claims::new(sub, value),
            &jsonwebtoken::EncodingKey::from_secret(self.encoding_key.as_ref()),
        )?)
    }

    pub fn decode(&self, token: &str) -> Result<(String, Value)> {
        let claims = jsonwebtoken::decode::<Claims>(
            token,
            &jsonwebtoken::DecodingKey::from_secret(self.encoding_key.as_ref()),
            &jsonwebtoken::Validation::default(),
        )?
        .claims;

        Ok((claims.sub, claims.value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::encoding_key::generate_encoding_key;

    #[test]
    fn test_jwt_with_valid_claim() {
        let test_key = generate_encoding_key();
        let subject = "test";

        let jwt = JWT::new(test_key);
        let value = serde_json::json!({"test": "value"});

        let token = jwt.encode(subject, value.clone()).unwrap();

        let actual = jwt.decode(&token).unwrap();

        assert_eq!(actual.0, subject.to_string());
        assert_eq!(actual.1, value);
    }
}

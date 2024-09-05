use super::{claims::Claims, Result};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header};
use serde_json::{json, Value};

// How long the token is valid for in seconds.
const LIFETIME: u64 = 60 * 60 * 24 * 7 * 30;

// The algorithm used to encode/decode the JWT.
const ALGORITHM: Algorithm = Algorithm::RS256;

/// Encode the given value using the given encoding key.
pub fn encode(sub: String, value: Value, key: &EncodingKey) -> Result<String> {
    let header = Header::new(ALGORITHM);
    let claims = Claims::new(sub, value, LIFETIME);

    Ok(jsonwebtoken::encode(&header, &claims, key)?)
}

/// Decode the given token using the given decoding key.
pub fn decode(token: &str, key: &DecodingKey) -> Result<Value> {
    let mut validation = jsonwebtoken::Validation::new(ALGORITHM);

    Ok(jsonwebtoken::decode::<Claims>(token, key, &validation)?
        .claims
        .body())
}

#[cfg(test)]
mod tests {
    use super::*;
    use openssl::rsa::Rsa;

    #[test]
    fn test_encoding_and_decoding() {
        let encoding_key =
            EncodingKey::from_rsa_pem(include_bytes!("test_private_key.pem")).unwrap();
        let decoding_key =
            DecodingKey::from_rsa_pem(include_bytes!("test_public_key.pem")).unwrap();

        let subject = "test".to_string();
        let value = json!({"hello": "world"});

        let token = encode(subject, value.clone(), &encoding_key).unwrap();
        let decoded_value = decode(&token, &decoding_key).unwrap();

        assert_eq!(value, decoded_value);
    }
}

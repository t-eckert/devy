use super::session::Session;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// The internal claims structure for the JWT.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    sub: String,
    body: Value,
    iss: String,
    exp: u64,
    iat: u64,
    nbf: u64,
}

impl Claims {
    /// Create a new set of claims with the body that is valid for the given lifetime.
    pub fn new(sub: String, body: Value, lifetime: u64) -> Self {
        Self {
            sub,
            body,
            iss: "https://api.devy.page".to_string(),
            exp: jsonwebtoken::get_current_timestamp() + lifetime,
            iat: jsonwebtoken::get_current_timestamp(),
            nbf: jsonwebtoken::get_current_timestamp(),
        }
    }

    /// Get the body of the claims.
    pub fn body(&self) -> Value {
        self.body.clone()
    }

    /// Check if the claims are expired.
    pub fn is_expired(&self) -> bool {
        self.exp <= jsonwebtoken::get_current_timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jwt_is_not_expired_with_long_lifetime() {
        let sub = "test".to_string();
        let value = serde_json::json!({"test": "value"});
        let claims = Claims::new(sub, value, 3600000000);
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_jwt_is_expired_with_short_lifetime() {
        let sub = "test".to_string();
        let value = serde_json::json!({"test": "value"});
        let claims = Claims::new(sub, value, 0);
        assert!(claims.is_expired());
    }
}

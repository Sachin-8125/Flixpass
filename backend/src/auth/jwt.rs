use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

use crate::{error::ApiError, types::Role};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub role: String,
    pub exp: usize,
}

pub fn issue_token(
    secret: &str,
    user_id: &str,
    email: &str,
    role: Role,
) -> Result<String, ApiError> {
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        role: role.as_db().to_string(),
        exp: (Utc::now() + Duration::hours(12)).timestamp() as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(ApiError::from)
}

pub fn decode_token(secret: &str, token: &str) -> Result<Claims, ApiError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| ApiError::unauthorized("Invalid or expired token."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issued_token_decodes_to_expected_claims() {
        let token = issue_token("test-secret", "user-1", "user@example.com", Role::Admin)
            .expect("token should be issued");

        let claims = decode_token("test-secret", &token).expect("token should decode");

        assert_eq!(claims.sub, "user-1");
        assert_eq!(claims.email, "user@example.com");
        assert_eq!(claims.role, "ADMIN");
    }

    #[test]
    fn token_rejects_wrong_secret() {
        let token = issue_token("test-secret", "user-1", "user@example.com", Role::Customer)
            .expect("token should be issued");

        let result = decode_token("wrong-secret", &token);

        assert!(result.is_err());
    }
}
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use crate::error::ApiError;

pub fn hash_password(password: &str) -> Result<String, ApiError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| ApiError::Internal(anyhow::anyhow!("failed to hash password")))
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), ApiError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|_| ApiError::unauthorized("Invalid email or password."))?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .map_err(|_| ApiError::unauthorized("Invalid email or password."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_hash_verifies_original_password_only() {
        let hash = hash_password("correct horse battery staple").expect("password should hash");

        assert!(verify_password("correct horse battery staple", &hash).is_ok());
        assert!(verify_password("incorrect password", &hash).is_err());
    }
}
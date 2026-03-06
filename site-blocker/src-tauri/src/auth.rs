use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Errore hashing: {0}")]
    Hash(String),
    #[error("PIN errato")]
    WrongPin,
}

pub fn hash_pin(pin: &str) -> Result<String, AuthError> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(pin.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| AuthError::Hash(e.to_string()))
}

pub fn verify_pin(pin: &str, stored_hash: &str) -> Result<(), AuthError> {
    let parsed = PasswordHash::new(stored_hash)
        .map_err(|e| AuthError::Hash(e.to_string()))?;
    Argon2::default()
        .verify_password(pin.as_bytes(), &parsed)
        .map_err(|_| AuthError::WrongPin)
}

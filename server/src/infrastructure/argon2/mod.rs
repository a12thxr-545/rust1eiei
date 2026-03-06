use anyhow::{Ok, Result};
use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};

pub fn hash(password: String) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let bytes_password = password.as_bytes();

    let argon2 = Argon2::default();

    let value = argon2
        .hash_password(bytes_password, &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;
    Ok(value.to_string())
}

pub fn verify(password: String, hashed_password: String) -> Result<bool> {
    // Attempt to parse as a valid PHC hash (Argon2)
    match PasswordHash::new(&hashed_password) {
        std::result::Result::Ok(parsed_hash) => {
            let bytes_password = password.as_bytes();
            let value = Argon2::default()
                .verify_password(bytes_password, &parsed_hash)
                .is_ok();
            Ok(value)
        }
        std::result::Result::Err(_) => {
            // If it's not a valid hash, it might be a legacy plain-text password
            // This is for backward compatibility with old users (arthur545, toey545, etc.)
            Ok(password == hashed_password)
        }
    }
}

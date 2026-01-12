use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use super::models::Claims;
use std::time::{SystemTime, UNIX_EPOCH};

const SECRET: &str = "super_secret_key_change_me_in_prod";

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?.to_string();
    Ok(password_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(password_hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub fn create_jwt(username: &str, role: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() + 3600; // 1 hour

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
        role: role.to_owned(),
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET.as_ref()))
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

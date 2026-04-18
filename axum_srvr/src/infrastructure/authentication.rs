use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tracing::{info};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: u64,
    pub exp: usize,
    pub roles: Vec<String>,
}


impl<S> FromRequestParts<S> for Claims
    where S: Send + Sync
{
    type Rejection = StatusCode;
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        {
            info!("from_request_parts");

            let auth = parts
                .headers
                .get("Authorization")
                .and_then(|s| s.to_str().ok())
                .ok_or(StatusCode::UNAUTHORIZED)?;

            let token = auth
                .strip_prefix("Bearer")
                .ok_or(StatusCode::UNAUTHORIZED)?;
            
            let secret =
                std::env::var("JWT_SECRET").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            info!("from_request_parts secret {}", &secret);

            //added trim() to the token on AI suggestion
            // fixed the "Base64 error: Invalid symbol 32, offset 0." error 
            let data =  decode::<Claims>(
                token.trim(),
                &DecodingKey::from_secret(secret.as_bytes()),
                &Validation::default(),
            ).map_err(|_| StatusCode::UNAUTHORIZED)?;

            Ok(data.claims)
        }
    }
}

pub fn generate_jwt(user_id: u64, secret: &String, roles: Vec<String>) -> String {
    let claims = Claims {
        sub: user_id,
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        roles,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

pub fn hash_password(passwd: &str) -> Result<String, argon2::Error> {
    let salt = SaltString::generate(&mut OsRng);
    // Argon2::default() uses Argon2id
    let argon2 = Argon2::default();

    // Hash password
    let hashed_passwd = argon2.hash_password(passwd.as_bytes(), &salt).unwrap();
    Ok(hashed_passwd.to_string())
}

pub fn verify_password(passwd_hash: &str, password: &String) -> bool {
    let parsed_hash = PasswordHash::new(&passwd_hash).unwrap();
    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        return true;
    }
    false
}

pub fn credential_check(passwd: &str, stored_hash: &str) -> bool {
    let parsed_hash = PasswordHash::new(stored_hash).expect("Failed to parse");
    let argon2 = Argon2::default();

    argon2
        .verify_password(passwd.as_bytes(), &parsed_hash)
        .is_ok()
}

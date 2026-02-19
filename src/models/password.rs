use serde::{Deserialize, Serialize};

/// DB model
#[derive(Debug)]
pub struct PasswordRecord {
    pub id: String,
    pub encrypted_pass: String,
}

/// API request body
#[derive(Deserialize)]
pub struct PasswordRequest {
    pub id: String,
    pub password: String,
}

/// API response — always decrypted
#[derive(Serialize)]
pub struct PasswordResponse {
    pub id: String,
    pub password: String,
}
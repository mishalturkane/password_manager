use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use std::env;

fn get_key() -> String {
    env::var("ENCRYPTION_KEY").unwrap_or_else(|_| "default_secret_key_32".to_string())
}

/// Encrypt plain text password
pub fn encrypt(plain: &str) -> String {
    let mc = new_magic_crypt!(get_key(), 256);
    mc.encrypt_str_to_base64(plain)
}

/// Decrypt encrypted password
pub fn decrypt(encrypted: &str) -> String {
    let mc = new_magic_crypt!(get_key(), 256);
    mc.decrypt_base64_to_string(encrypted)
        .unwrap_or_else(|_| "❌ Decryption failed".to_string())
}
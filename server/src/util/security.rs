use rand::{distributions::Alphanumeric, Rng};
use bcrypt::{hash, verify, DEFAULT_COST};
use sha2::{Sha256, Digest};
use jsonwebtoken::*;

pub fn generate_salt() -> String {
    let rng = rand::thread_rng();
    let salt: String = rng.sample_iter(&Alphanumeric).take(32).map(char::from).collect();
    salt
}

pub fn generate_password_hash(password: String, salt: String) -> String {
    match hash(format!("{}{}", password, salt), DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => String::from(""),
    }
}

pub fn verify_password(password: String, salt: String, hash: String) -> bool {
    verify(format!("{}{}", password, salt), &hash).unwrap()
}
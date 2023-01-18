use rand::{distributions::Alphanumeric, Rng, thread_rng};
use bcrypt::{hash, verify};
use hmac::{Hmac, Mac};
use jwt::{AlgorithmType, Header, SignWithKey, Token};
use sha2::Sha256;
use std::collections::BTreeMap;
use std::time::SystemTime;

pub fn generate_salt() -> String {
    let rng = thread_rng();
    rng.sample_iter(&Alphanumeric).take(32).map(char::from).collect()
}

pub fn generate_password_hash(password: &str, salt: &str) -> String {
    hash(format!("{}{}", password, salt), 4).unwrap()
}

pub fn verify_password_hash(password: &str, salt: &str, hash: &str) -> bool {
    verify(format!("{}{}", password, salt), hash).unwrap()
}

pub fn generate_access_token(uid: &str, role: &str) -> String {
    dotenv::dotenv().ok();
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();

    let mut claims = BTreeMap::new();
    claims.insert("exp", (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + (60 * 10)).to_string());
    claims.insert("iat", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string());
    claims.insert("iss", "studentifier-server".to_string());
    claims.insert("sub", "access_token".to_string());
    claims.insert("uid", uid.to_string());
    claims.insert("role", role.to_string());

    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };

    let key = Hmac::<Sha256>::new_from_slice(jwt_secret.as_bytes()).unwrap();

    Token::new(header, claims).sign_with_key(&key).unwrap().as_str().to_string()
}

pub fn generate_refresh_token(uid: &str, role: &str) -> String {
    let jwt_secret = std::env::var("JWT_SECRET").unwrap();

    let mut claims = BTreeMap::new();
    claims.insert("exp", (SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() + (60 * 60 * 12)).to_string());
    claims.insert("iat", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string());
    claims.insert("iss", "studentifier-server".to_string());
    claims.insert("sub", "refresh_token".to_string());
    claims.insert("uid", uid.to_string());
    claims.insert("role", role.to_string());

    let header = Header {
        algorithm: AlgorithmType::Hs256,
        ..Default::default()
    };

    let key = Hmac::<Sha256>::new_from_slice(jwt_secret.as_bytes()).unwrap();

    Token::new(header, claims).sign_with_key(&key).unwrap().as_str().to_string()
}
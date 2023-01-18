use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::VerifyWithKey;
use sha2::Sha256;

pub struct AuthService {} impl AuthService {
    pub fn get_claims(token: String) -> BTreeMap<String, String> {
        let jwt_secret = std::env::var("JWT_SECRET").unwrap();
        let jwt_key = Hmac::<Sha256>::new_from_slice(jwt_secret.as_bytes()).unwrap();

        let claims: BTreeMap<String, String> = token.verify_with_key(&jwt_key).unwrap();

        return claims;
    }
}
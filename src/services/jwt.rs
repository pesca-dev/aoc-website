use std::collections::BTreeMap;
use std::env;
use std::error::Error;

use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

fn key() -> Result<Hmac<Sha256>, Box<dyn Error>> {
    let key = env::var("JWT_KEY").expect("JWT key should be given");
    Ok(Hmac::new_from_slice(key.as_bytes())?)
}

#[tracing::instrument(level = "trace")]
pub fn sign(claims: BTreeMap<String, String>) -> Result<String, Box<dyn Error>> {
    let key = key()?;

    Ok(claims.sign_with_key(&key)?)
}

#[tracing::instrument(level = "trace")]
pub fn extract(token: String) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let key = key()?;

    Ok(token.verify_with_key(&key)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::{collections::BTreeMap, env};

    #[test]
    fn test_jwt_sign() {
        env::set_var("JWT_KEY", "some-key");

        let mut claims = BTreeMap::new();
        claims.insert("sub".to_string(), "some_user".to_string());
        assert!(sign(claims).is_ok())
    }

    #[test]
    fn test_jwt_extract() {
        env::set_var("JWT_KEY", "some-key");

        let mut claims = BTreeMap::new();
        claims.insert("sub".to_string(), "some_user".to_string());

        let token = sign(claims).unwrap();
        let claims = extract(token).unwrap();

        assert_eq!(claims["sub"], "some_user".to_string());
    }
}

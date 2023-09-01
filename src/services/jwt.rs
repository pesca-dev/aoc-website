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

pub fn sign(claims: BTreeMap<String, String>) -> Result<String, Box<dyn Error>> {
    let key = key()?;

    Ok(claims.sign_with_key(&key)?)
}

pub fn extract(token: String) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    let key = key()?;

    Ok(token.verify_with_key(&key)?)
}

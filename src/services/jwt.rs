use std::env;
use std::error::Error;

use hmac::digest::KeyInit;
use hmac::Hmac;
use jwt::{FromBase64, SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct VerifyJWT {
    pub sub: String,
    pub exp: i64,
}

fn key() -> Result<Hmac<Sha256>, Box<dyn Error>> {
    let key = env::var("JWT_KEY").expect("JWT key should be given");
    Ok(Hmac::new_from_slice(key.as_bytes())?)
}

#[tracing::instrument(level = "trace")]
pub fn sign<A>(claims: A) -> Result<String, Box<dyn Error>>
where
    A: SignWithKey<String> + std::fmt::Debug,
{
    let key = key()?;

    Ok(claims.sign_with_key(&key)?)
}

#[tracing::instrument(level = "trace")]
pub fn extract<T>(token: String) -> Result<T, Box<dyn Error>>
where
    T: FromBase64,
{
    let key = key()?;

    Ok(token.verify_with_key(&key)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;

    #[test]
    fn test_jwt_sign() {
        env::set_var("JWT_KEY", "some-key");

        let claims = VerifyJWT {
            sub: "some_user".to_string(),
            exp: 0,
        };
        assert!(sign(claims).is_ok())
    }

    #[test]
    fn test_jwt_extract() {
        env::set_var("JWT_KEY", "some-key");

        let claims = VerifyJWT {
            sub: "some_user".to_string(),
            exp: 0,
        };

        let token = sign(claims).unwrap();
        let claims: VerifyJWT = extract(token).unwrap();

        assert_eq!(claims.sub, "some_user".to_string());
    }
}

mod login;
mod logout;
mod register;
mod verify;

pub use self::login::*;
pub use self::logout::*;
pub use self::register::*;
pub use self::verify::*;

use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use std::error::Error;
    use chrono::Duration;
    use crate::services::{mail::Mail, jwt, jwt::VerifyJWT};

    #[tracing::instrument(level = "trace")]
    fn create_jwt(username: &str) -> Result<String, Box<dyn Error>> {
        tracing::debug!("creating jwt");
        let claims = VerifyJWT {
            sub: username.to_string(),
            exp: (chrono::Utc::now().naive_local() + Duration::minutes(15)).timestamp(),
        };
        jwt::sign(claims)
    }

    #[tracing::instrument(level = "trace")]
    fn send_verification_mail(username: String, email: String, token: String) -> Result<(), Box<dyn Error>> {
        tracing::debug!("sending verification mail for '{username}' to '{email}'");
        let mail = Mail {
            subject: Some("Registration Mail".into()),
            recipient: email,
            content: Some(format!("Hey {username}! \nThank you for registering! To complete your registration, please use the following link: https://aoc.inf-cau.de/verify?token={token}"))
        };

        mail.send()
    }
}
}

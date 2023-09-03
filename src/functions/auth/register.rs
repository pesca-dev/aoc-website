use leptos::*;
use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::{
    functions::auth::{create_jwt, send_verification_mail},
    model::User,
    utils::password::hash_password,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RegistrationResult {
    Ok,
    InternalServerError,
    PasswordsDoNotMatch,
    CredentialsAlreadyTaken,
}

impl Display for RegistrationResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RegistrationResult::*;

        match self {
            Ok => f.write_str("Registration Successful"),
            InternalServerError => f.write_str("Internal Server Error"),
            PasswordsDoNotMatch => f.write_str("Passwords do not match"),
            CredentialsAlreadyTaken => f.write_str("Credentials are already taken"),
        }
    }
}

#[tracing::instrument(level = "trace", skip(cx, password, password_confirm))]
#[server(Register, "/api")]
pub async fn register(
    cx: Scope,
    username: String,
    password: String,
    password_confirm: String,
    email: String,
) -> Result<RegistrationResult, ServerFnError> {
    tracing::debug!("attempting to register user...");
    if password != password_confirm {
        return Ok(RegistrationResult::PasswordsDoNotMatch);
    }

    // create JWT for verification mail
    let token = match create_jwt(&username) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("failed to create JWT: {e:#?}");
            return Ok(RegistrationResult::InternalServerError);
        }
    };

    if let Err(e) = (User {
        username: username.clone(),
        password: hash_password(password)?,
        email: email.clone(),
        ..Default::default()
    })
    .create()
    .await
    .map_err(|e| {
        tracing::error!("{e:#?}");
        RegistrationResult::CredentialsAlreadyTaken
    }) {
        return Ok(e);
    };

    if send_verification_mail(username, email, token).is_err() {
        return Ok(RegistrationResult::InternalServerError);
    }

    Ok(RegistrationResult::Ok)
}

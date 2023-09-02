use std::fmt::Display;

use cfg_if::cfg_if;

use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use std::error::Error;
    use std::collections::BTreeMap;
    use actix_identity::IdentityExt;
    use crate::hooks::use_identity;
    use crate::utils::password::hash_password;
    use crate::model::{User, LoginError, Session};
    use crate::services::{mail::Mail, jwt};

    #[tracing::instrument(level = "trace")]
    fn create_jwt(username: &str) -> Result<String, Box<dyn Error>> {
        tracing::debug!("creating jwt");
        let mut claims = BTreeMap::new();
        claims.insert("sub".into(), username.to_string());
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LoginResult {
    Ok,
    InternalServerError,
    WrongCredentials,
    VerifyEmail,
    AlreadyLoggedIn,
}

impl Display for LoginResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use LoginResult::*;

        match self {
            Ok => f.write_str("Login Successful"),
            InternalServerError => f.write_str("Internal Server Error"),
            WrongCredentials => f.write_str("Wrong Credentials"),
            VerifyEmail => f.write_str("Verify your Email before logging in"),
            AlreadyLoggedIn => f.write_str("You are already logged in"),
        }
    }
}

#[tracing::instrument(level = "trace", skip(cx, password))]
#[server(Login, "/api")]
pub async fn login(
    cx: Scope,
    username: String,
    password: String,
) -> Result<LoginResult, ServerFnError> {
    let Some(req) = use_context::<actix_web::HttpRequest>(cx) else {
        return Ok(LoginResult::InternalServerError);
    };

    let ident = IdentityExt::get_identity(&req);

    if ident.is_ok() {
        leptos_actix::redirect(cx, "/");
        return Ok(LoginResult::AlreadyLoggedIn);
    }

    let user: Option<User> = User::get_by_username(&username).await;

    let Some(mut user) = user else {
        return Ok(LoginResult::WrongCredentials);
    };

    if !user.email_verified {
        return Ok(LoginResult::VerifyEmail);
    }

    match user.login(&password, &req).await {
        Err(LoginError::Internal) => return Ok(LoginResult::InternalServerError),
        Err(LoginError::PasswordMismatch) => return Ok(LoginResult::WrongCredentials),
        Ok(_) => (),
    };

    leptos_actix::redirect(cx, "/");
    return Ok(LoginResult::Ok);
}

#[tracing::instrument(level = "trace", skip(cx))]
#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let Ok(identity) = use_identity(cx) else {
        return Ok(());
    };

    let session_id = identity.id().expect("session did not have an id");
    Session::destroy(&session_id).await;

    identity.logout();

    Ok(())
}

#[derive(Serialize, Deserialize)]
pub enum VerificationResult {
    Ok,
    InvalidToken,
    InternalServerError,
}

#[tracing::instrument(level = "trace", skip(cx))]
#[server(Verify, "/api")]
pub async fn verify_user(cx: Scope, token: String) -> Result<VerificationResult, ServerFnError> {
    let payload = match jwt::extract(token) {
        Ok(data) => data,
        Err(e) => {
            tracing::warn!("failed to extract JWT: {e:#?}");
            return Ok(VerificationResult::InternalServerError);
        }
    };

    let username = match payload.get("sub") {
        Some(username) => username,
        None => {
            return Ok(VerificationResult::InvalidToken);
        }
    };

    let user = match User::get_by_username(username).await {
        Some(user) => user,
        None => {
            return Ok(VerificationResult::InvalidToken);
        }
    };

    user.verify_email().await;

    Ok(VerificationResult::Ok)
}

#[tracing::instrument(level = "trace", skip(cx))]
#[server(ResendVerification, "/api")]
pub async fn resend_verification_mail(cx: Scope, username: String) -> Result<(), ServerFnError> {
    let Some(user) = User::get_by_username(&username).await else {
        return Ok(());
    };

    let Ok(token) = create_jwt(&username) else {
        return Ok(());
    };

    let _ = send_verification_mail(username, user.email, token);
    Ok(())
}

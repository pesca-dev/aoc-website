use std::fmt::Display;

use cfg_if::cfg_if;

use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use actix_identity::IdentityExt;
    use crate::hooks::use_identity;
    use crate::utils::password::hash_password;
    use crate::model::{User, LoginError, Session};
    use crate::services::mail::Mail;
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

#[server(Register, "/api")]
pub async fn register(
    cx: Scope,
    username: String,
    password: String,
    password_confirm: String,
    email: String,
) -> Result<RegistrationResult, ServerFnError> {
    if password != password_confirm {
        return Ok(RegistrationResult::PasswordsDoNotMatch);
    }

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

    let mail = Mail {
        subject: Some("Registration Mail".into()),
        recipient: email,
        content: Some(format!("Hey {username}! \nThank you for registering! To complete your registration, please use the following link: TODO"))
    };

    if mail.send().is_err() {
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
            VerifyEmail => f.write_str("Verify your Email bevore logging in"),
            AlreadyLoggedIn => f.write_str("You are already logged in"),
        }
    }
}

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

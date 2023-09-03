use std::fmt::Display;

#[cfg(feature = "ssr")]
use actix_identity::IdentityExt;

use leptos::*;
use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use crate::model::{LoginError, User};

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

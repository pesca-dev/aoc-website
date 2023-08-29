use std::fmt::Display;

use cfg_if::cfg_if;

use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use actix_identity::{Identity, IdentityExt};
    use actix_web::{
        HttpMessage,
    };
    use crate::hooks::use_identity;
    use crate::utils::password::{verify_password, hash_password};
    use crate::model::{User, Session};
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
            Ok => f.write_str("Registration Successful!"),
            InternalServerError => f.write_str("Internal Server Error"),
            PasswordsDoNotMatch => f.write_str("Passwords do not match!"),
            CredentialsAlreadyTaken => f.write_str("Credentials are already taken!"),
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
        username,
        password: hash_password(password)?,
        email,
        ..Default::default()
    })
    .create()
    .await
    .map_err(|_| RegistrationResult::CredentialsAlreadyTaken)
    {
        return Ok(e);
    };

    Ok(RegistrationResult::Ok)
}

#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<actix_web::HttpRequest>(cx) else {
        return Err(ServerFnError::MissingArg(
            "Failed to get the Request".to_string(),
        ));
    };

    let ident = IdentityExt::get_identity(&req);

    if ident.is_ok() {
        leptos_actix::redirect(cx, "/");
        return Err(ServerFnError::ServerError(
            "User is already logged in...".to_string(),
        ));
    }

    let user: Option<User> = User::get_by_username(&username).await;

    let Some(mut user) = user else {
        return Err(ServerFnError::ServerError("User not found".into()));
    };

    let Ok(true) = verify_password(&password, &user.password) else {
        return Err(ServerFnError::ServerError("User not found".into()));
    };

    let Some(session_id) = user.login().await else {
        return Err(ServerFnError::ServerError("Some Error".into()));
    };

    Identity::login(&req.extensions(), session_id).unwrap();

    leptos_actix::redirect(cx, "/");
    return Ok(());
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let Ok(identity) = use_identity(cx) else {
        return Ok(());
    };

    let session_id = identity.id().expect("session did not have an error");
    Session::destroy(&session_id).await;

    identity.logout();

    Ok(())
}

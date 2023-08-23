use cfg_if::cfg_if;

use leptos::*;
use serde::{Deserialize, Serialize};

cfg_if! {
if #[cfg(feature = "ssr")] {
    use actix_identity::{Identity, IdentityExt};
    use actix_web::{
        HttpMessage,
    };
    use crate::hooks::{use_database, use_identity};
    use crate::utils::password::{verify_password, hash_password};
}
}

#[derive(Debug, Deserialize, Serialize)]
struct User {
    username: String,
    password: String,
    email: String,
}

#[server(Register, "/api")]
pub async fn register(
    cx: Scope,
    username: String,
    password: String,
    email: String,
) -> Result<(), ServerFnError> {
    let db = use_database("test").await;

    let created: Option<User> = db
        .create(("user", &username))
        .content(User {
            username,
            password: hash_password(password)?,
            email,
        })
        .await?;

    log!("Whoop: {created:#?}");

    Ok(())
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

    let db = use_database("test").await;

    let user: Option<User> = db.select(("user", &username)).await?;

    let Some(user) = user else {
        return Err(ServerFnError::ServerError("User not found".into()));
    };

    let Ok(true) = verify_password(password, user.password) else {
        return Err(ServerFnError::ServerError("User not found".into()));
    };

    Identity::login(&req.extensions(), username.clone()).unwrap();
    leptos_actix::redirect(cx, "/");
    return Ok(());
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let Ok(identity) = use_identity(cx) else {
        return Ok(());
    };

    identity.logout();

    Ok(())
}

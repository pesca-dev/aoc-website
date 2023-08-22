use cfg_if::cfg_if;

use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use actix_identity::{Identity, IdentityExt};
    use actix_web::{
        HttpMessage,
    };
    use crate::hooks::use_identity;
}
}

#[server(Login, "/api")]
pub async fn login(cx: Scope, username: String, password: String) -> Result<(), ServerFnError> {
    let Some(req) = use_context::<actix_web::HttpRequest>(cx) else {
        return Err(ServerFnError::MissingArg(
            "Failed to get the Request".to_string(),
        ));
    };

    let ident = IdentityExt::get_identity(&req);

    if ident.is_err() {
        Identity::login(&req.extensions(), username.clone()).unwrap();
    }

    leptos_actix::redirect(cx, "/");
    Ok(())
}

#[server(Logout, "/api")]
pub async fn logout(cx: Scope) -> Result<(), ServerFnError> {
    let Ok(identity) = use_identity(cx) else {
        return Ok(());
    };

    identity.logout();

    Ok(())
}

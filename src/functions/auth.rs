use cfg_if::cfg_if;

use leptos::*;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use actix_identity::{Identity, IdentityExt};
    use actix_web::{
        HttpMessage,
    };

    pub fn use_identity(req: &actix_web::HttpRequest) -> Result<Identity, ServerFnError> {
        IdentityExt::get_identity(req).map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
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
    let Some(req) = use_context::<actix_web::HttpRequest>(cx) else {
        return Ok(());
    };

    let identity = use_identity(&req)?;

    identity.logout();

    Ok(())
}

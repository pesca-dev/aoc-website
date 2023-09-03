use leptos::*;

#[cfg(feature = "ssr")]
use crate::{hooks::use_identity, model::Session};

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

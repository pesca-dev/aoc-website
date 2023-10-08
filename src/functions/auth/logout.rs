use leptos::*;

#[cfg(feature = "ssr")]
use crate::{hooks::use_identity, model::Session};

#[tracing::instrument(level = "trace")]
#[server]
pub async fn logout() -> Result<(), ServerFnError> {
    let Ok(identity) = use_identity() else {
        return Ok(());
    };

    let session_id = identity.id().expect("session did not have an id");
    Session::destroy(&session_id).await;

    identity.logout();

    Ok(())
}

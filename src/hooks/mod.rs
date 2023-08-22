use cfg_if::cfg_if;

use leptos::*;

use crate::contexts::AuthContext;

cfg_if! {
if #[cfg(feature = "ssr")] {
    use actix_identity::{Identity, IdentityExt};

    /// Try to get the identity depending on the current context.
    pub fn use_identity(cx: Scope) -> Result<Identity, ServerFnError> {
        let Some(req) = use_context::<actix_web::HttpRequest>(cx) else {
            return Err(ServerFnError::MissingArg(
                "Failed to get the Request".to_string(),
            ));
        };

        IdentityExt::get_identity(&req).map_err(|e| ServerFnError::ServerError(e.to_string()))
    }
}
}

/// Get the auth context.
///
/// This function will panic if there is no AuthContext provided in an upper component.
pub fn use_auth(cx: Scope) -> AuthContext {
    use_context::<AuthContext>(cx).expect("no valid AuthContext given!")
}

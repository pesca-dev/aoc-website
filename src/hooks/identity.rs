use actix_identity::{Identity, IdentityExt};
use leptos::*;

/// Try to get the identity depending on the current context.
pub fn use_identity(cx: Scope) -> Result<Identity, ServerFnError> {
    let Some(req) = use_context::<actix_web::HttpRequest>(cx) else {
        return Err(ServerFnError::MissingArg(
            "Failed to get the Request".to_string(),
        ));
    };

    IdentityExt::get_identity(&req).map_err(|e| ServerFnError::ServerError(e.to_string()))
}

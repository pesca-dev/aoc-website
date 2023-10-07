use actix_identity::{Identity, IdentityExt};
use leptos::*;

/// Try to get the identity depending on the current context.
#[tracing::instrument(level = "trace")]
pub fn use_identity() -> Result<Identity, ServerFnError> {
    let Some(req) = use_context::<actix_web::HttpRequest>() else {
        return Err(ServerFnError::MissingArg(
            "Failed to get the Request".to_string(),
        ));
    };

    IdentityExt::get_identity(&req).map_err(|e| ServerFnError::ServerError(e.to_string()))
}

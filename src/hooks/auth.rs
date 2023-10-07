use leptos::*;

use crate::contexts::AuthContext;

/// Get the auth context.
///
/// This function will panic if there is no AuthContext provided in an upper component.
#[tracing::instrument(level = "trace")]
pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().expect("no valid AuthContext given!")
}

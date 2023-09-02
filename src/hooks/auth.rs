use leptos::*;

use crate::contexts::AuthContext;

/// Get the auth context.
///
/// This function will panic if there is no AuthContext provided in an upper component.
#[tracing::instrument(level = "trace", skip(cx))]
pub fn use_auth(cx: Scope) -> AuthContext {
    use_context::<AuthContext>(cx).expect("no valid AuthContext given!")
}

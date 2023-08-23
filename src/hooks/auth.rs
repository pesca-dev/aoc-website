use leptos::*;

use crate::contexts::AuthContext;

/// Get the auth context.
///
/// This function will panic if there is no AuthContext provided in an upper component.
pub fn use_auth(cx: Scope) -> AuthContext {
    use_context::<AuthContext>(cx).expect("no valid AuthContext given!")
}

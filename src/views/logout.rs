use leptos::*;
use leptos_router::Redirect;

use crate::{functions::Logout, hooks::use_auth};

#[component]
pub fn LogoutView(cx: Scope) -> impl IntoView {
    let auth = use_auth(cx);
    auth.logout.dispatch(Logout {});

    view! { cx,
        <div>
            <Redirect path="/" />
        </div>
    }
}

use leptos::*;
use leptos_router::Redirect;

use crate::{contexts::use_auth, functions::Logout};

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

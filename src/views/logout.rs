use leptos::*;
use leptos_router::Redirect;

use crate::{functions::Logout, hooks::use_auth};

#[component]
pub fn LogoutView() -> impl IntoView {
    let auth = use_auth();
    auth.logout.dispatch(Logout {});

    view! {
        <div>
            <Redirect path="/" />
        </div>
    }
}

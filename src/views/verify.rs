use leptos::*;
use leptos_router::use_query_map;

use crate::{functions::Verify, hooks::use_auth};

#[component]
pub fn VerifyView(cx: Scope) -> impl IntoView {
    let auth = use_auth(cx);

    let query = use_query_map(cx).get_untracked();
    let token = query.get("token").unwrap().clone();

    create_effect(cx, move |_| {
        auth.verify.dispatch(Verify {
            token: token.clone(),
        });
    });

    let return_value = move || match auth.verify.value().get() {
        Some(Ok(message)) => Some(message),
        _ => None,
    };

    let message = move || {
        return_value()
            .map(|msg| msg.to_string())
            .unwrap_or("".into())
    };

    view! { cx,
        <section>
            <h2>{message}</h2>
        </section>
    }
}

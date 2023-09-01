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

    view! { cx,
        <section>
            <h1>"Success"</h1>
            <p>"You can now login!"</p>
        </section>
    }
}

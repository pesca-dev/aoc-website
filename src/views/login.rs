use leptos::*;
use leptos_router::ActionForm;

use crate::hooks::use_auth;

#[component]
pub fn LoginView(cx: Scope) -> impl IntoView {
    let auth = use_auth(cx);

    view! { cx,
        <Transition
            fallback=move || ()>
            {move || {
                let condition = move || {
                    let user = auth.user.read(cx);
                    !matches!(user, Some(Ok(_)))
                };
                view!{ cx,
                    <Show
                        when=condition
                        fallback=|cx| view! { cx, <section>"Logged in"</section>}>
                        <section class="login-view">
                            <ActionForm action=auth.login>
                                <h1>"Login"</h1>
                                <label>
                                    <span>"Username:"</span>
                                    <input type="text" name="username" />
                                </label>
                                <label>
                                    <span>"Password:"</span>
                                    <input type="password" name="password" />
                                </label>
                                <button type="submit">"Login"</button>
                            </ActionForm>
                        </section>
                </Show>
                }
            }}
        </Transition>
    }
}

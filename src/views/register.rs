use leptos::*;
use leptos_router::ActionForm;

use crate::hooks::use_auth;

#[component]
pub fn RegisterView(cx: Scope) -> impl IntoView {
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
                        <section class="register-view">
                            <ActionForm action=auth.register>
                                <h1>"Register"</h1>
                                <label>
                                    <span>"Username"</span>
                                    <input type="text" name="username" required/>
                                </label>
                                <label>
                                    <span>"E-Mail"</span>
                                    <input type="email" name="email" required/>
                                </label>
                                <label>
                                    <span>"Password"</span>
                                    <input type="password" name="password" required/>
                                </label>
                                <label>
                                    <span>"Confirm Password"</span>
                                    <input type="password" name="password-confirm" required/>
                                </label>
                                <button type="submit" class="primary">"Login"</button>
                            </ActionForm>
                        </section>
                </Show>
                }
            }}
        </Transition>
    }
}

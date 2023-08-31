use leptos::*;
use leptos_router::ActionForm;

use crate::{functions::LoginResult, hooks::use_auth};

#[component]
pub fn LoginView(cx: Scope) -> impl IntoView {
    let auth = use_auth(cx);

    let result = move || {
        let result = auth.login.value();

        let msg: Option<LoginResult> = match result.get() {
            Some(return_value) => match return_value {
                Ok(result) => Some(result),
                Err(_) => Some(LoginResult::InternalServerError),
            },
            None => None,
        };
        msg
    };

    let message = move || {
        if let Some(result) = result() {
            result.to_string()
        } else {
            "".to_string()
        }
    };

    let is_ok = move || matches!(result(), Some(LoginResult::Ok));

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
                                <Show
                                    when=move || result().is_some()
                                    fallback=|cx| view! { cx, <span></span> }
                                >
                                    <div
                                        class="result"
                                        class:error=move || !is_ok()
                                        class:success=is_ok
                                    >
                                        {message()}
                                    </div>
                                </Show>
                                <h1>"Login"</h1>
                                <label>
                                    <span>"Username"</span>
                                    <input type="text" name="username" required/>
                                </label>
                                <label>
                                    <span>"Password"</span>
                                    <input type="password" name="password" required/>
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

use leptos::*;
use leptos_router::ActionForm;

use crate::{functions::RegistrationResult, hooks::use_auth};

#[component]
pub fn RegisterView(cx: Scope) -> impl IntoView {
    let auth = use_auth(cx);

    let condition = move || {
        let user = auth.user.read(cx);
        !matches!(user, Some(Ok(_)))
    };

    let result = move || {
        let result = auth.register.value();

        let msg: Option<RegistrationResult> = match result.get() {
            Some(return_value) => match return_value {
                Ok(result) => Some(result),
                Err(_) => Some(RegistrationResult::InternalServerError),
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

    let is_ok = move || matches!(result(), Some(RegistrationResult::Ok));

    view! { cx,
        <Transition
            fallback=move || ()>
            {move || {
                view!{ cx,
                    <Show
                        when=condition
                        fallback=|cx| view! { cx, <section>"Logged in"</section>}>
                        <section class="register-view">
                            <ActionForm action=auth.register>
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
                                    <input type="password" name="password_confirm" required/>
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

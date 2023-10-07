use leptos::*;
use leptos_router::*;

use crate::{app::CODE, components::CodeSidebar};

#[component]
pub fn CodeView() -> impl IntoView {
    let query = use_params_map();
    let user = move || query.with(|params| params.get("user").cloned().unwrap_or_default());

    view! {
        <CodeSidebar />
        <Show when=move || user().trim() != "" fallback=move || view! { <section>Select a user...</section>}>
            {Prism::highlight_all()}
            <section class="code-overview">
                <ul>
                    <li>
                        <div class="code-snippet">
                            <details open>
                                <summary>
                                    {user} Part 1
                                </summary>
                                <pre>
                                    <code class="language-rust">{CODE.trim()}</code>
                                </pre>
                            </details>
                        </div>
                    </li>
                    <li>
                        <div class="code-snippet">
                            <details>
                                <summary>
                                    {user} Part 2
                                </summary>
                                <pre>
                                    <code class="language-rust">{CODE.trim()}</code>
                                </pre>
                            </details>
                        </div>
                    </li>
                </ul>
            </section>
        </Show>
    }
}

#[allow(non_snake_case)]
mod Prism {
    #[cfg(feature = "hydrate")]
    use wasm_bindgen::prelude::wasm_bindgen;
    #[cfg(feature = "hydrate")]
    #[wasm_bindgen(module = "/js/prism.js")]
    extern "C" {
        pub fn highlight_all();
    }
    #[cfg(not(feature = "hydrate"))]
    #[allow(dead_code)]
    pub fn highlight_all() {}
}

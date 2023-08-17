use leptos::*;

#[component]
pub fn Sidebar(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <section class="sidebar">
            <div>
                {children(cx)}
            </div>
        </section>
    }
}

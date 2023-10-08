use leptos::*;

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    view! {
        <section class="sidebar">
            <div>
                {children()}
            </div>
        </section>
    }
}

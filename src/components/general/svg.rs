use leptos::*;

#[component]
pub fn Svg<'a>(cx: Scope, id: &'a str) -> impl IntoView {
    let url = format!("/assets/icons.svg#{id}");

    view! { cx,
        <svg viewBox="0 0 24 24">
            <use_ href=url/>
        </svg>
    }
}

use leptos::*;

#[component]
pub fn Svg<'a>(id: &'a str) -> impl IntoView {
    let url = format!("/assets/icons.svg#{id}");

    view! {
        <svg viewBox="0 0 24 24">
            <use_ href=url/>
        </svg>
    }
}

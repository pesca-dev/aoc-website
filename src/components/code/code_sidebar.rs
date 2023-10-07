use leptos::*;
use leptos_router::*;

use crate::components::Sidebar;

#[component]
pub fn CodeSidebar() -> impl IntoView {
    let params = use_params_map();
    let user = move || params.with(|params| params.get("user").cloned().unwrap_or_default());

    let names = vec![
        "h1ghbre4k3r",
        "dobiko",
        "dormanil",
        "maclement",
        "melf",
        "zihark",
        "sebfisch",
        "xtay2",
        "estugon",
        "fwcd",
        "b3z",
        "felioh",
        "h1tchhiker",
        "hendrick404",
        "tuhhy",
        "yorick",
        "skgland",
    ];

    let (users, _) = create_signal(names);

    view! {
        <Sidebar>
            <header><h3>Users</h3></header>
            <div class="day">
                <label for="day-select">Day</label>
                <select name="day" id="day-select">
                    <option value="1">1</option>
                    <option value="2">2</option>
                    <option value="3">3</option>
                    <option value="4">4</option>
                    <option value="5">5</option>
                    <option value="6">6</option>
                    <option value="7">7</option>
                    <option value="8">8</option>
                </select>
            </div>
            <ul>
                <For each=users key=|name| name.to_owned() children=move |name| {
                    let is_active = move || name == user();
                    let link = move || format!("/code/{name}");

                    view! {
                        <li>
                            <a href=link class:active=is_active>{name}</a>
                        </li>
                    }
                }/>
            </ul>
        </Sidebar>
    }
}

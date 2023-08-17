use leptos::*;

use crate::components::Svg;

#[component]
pub fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <div class="logo">
                <a href="/">
                    AoC
                </a>
            </div>
            <ul>
                <li>
                    <a href="/">
                        <span class="icon"><Svg id="home"/></span>
                        <span class="nav-label">Home</span>
                    </a>
                </li>
                <li>
                    <a href="/code">
                        <span class="icon"><Svg id="code-brackets"/></span>
                        <span class="nav-label">Code</span>
                    </a>
                </li>
                <li>
                    <a href="/last-years">
                        <span class="nav-label">Last Years</span>
                    </a>
                </li>
            </ul>
            <div class="profile">
                <details>
                    <summary>
                        <span class="nav-label">H1ghBre4k3r</span>
                        <span class="profile-picture">
                            <Svg id="user-circle" />
                        </span>
                    </summary>
                    <aside>
                        <ul>
                            <li>
                                <a href="/profile">
                                    <span class="icon"><Svg id="tools" /></span>Profile
                                </a>
                            </li>
                            <li>
                                <a href="/settings">
                                    <span class="icon"><Svg id="settings" /></span>Settings
                                </a>
                            </li>
                            <li>
                                <a href="/logout">
                                    <span class="icon"><Svg id="logout" /></span>Logout
                                </a>
                            </li>
                        </ul>
                    </aside>
                </details>
            </div>
        </nav>
    }
}

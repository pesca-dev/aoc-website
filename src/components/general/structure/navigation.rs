use leptos::*;

use crate::{components::Svg, hooks::use_auth};

#[component]
pub fn Navigation() -> impl IntoView {
    let auth = use_auth();

    view! {
        <nav>
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
                <Transition
                    fallback=move || ()>
                        {move || {
                            let user = auth.user.get();
                            view!{
                                <details>
                                    <summary>
                                        <span class="nav-label">{user.clone()}</span>
                                        <span class="profile-picture">
                                            <Svg id="user-circle" />
                                        </span>
                                    </summary>
                                    <aside>
                                    {move || {
                                        if let Some(Ok(_)) = user {
                                            view!{
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
                                            }
                                        } else {
                                            view!{
                                                <ul>
                                                    <li>
                                                        <a href="/login">
                                                            <span class="icon"><Svg id="login" /></span>Login
                                                        </a>
                                                    </li>
                                                    <li>
                                                        <a href="/register">
                                                            <span class="icon"><Svg id="register" /></span>Register
                                                        </a>
                                                    </li>
                                                </ul>
                                            }
                                        }
                                    }}
                                    </aside>
                                </details>
                        }
                    }}
                </Transition>
            </div>
        </nav>
    }
}

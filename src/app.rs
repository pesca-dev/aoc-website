use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/aoc_website.css"/>

        <Title text="Advent of Code"/>

        <Meta name="color-scheme" content="light" />

        <Router>
            <Navigation />
            <main>
                <Routes>
                    <Route path="" view=MainView/>
                    <Route path="/code" view=CodeView/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn MainView(cx: Scope) -> impl IntoView {
    view! { cx,
        <section>
            <h1>Advent of Code</h1>
        </section>
    }
}

#[component]
fn CodeView(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let user = move || query.with(|params| params.get("user").cloned().unwrap_or_default());

    view! { cx,
        <Sidebar />
        <section class="code-overview">
            <div class="code-snippet">
                {user}
            </div>
            <div class="code-snippet">
                Code Snippet
            </div>
            <div class="code-snippet">
                Code Snippet
            </div>
        </section>
    }
}

#[component]
fn Navigation(cx: Scope) -> impl IntoView {
    view! { cx,
        <nav>
            <div class="logo">
                <a href="/">
                    "LOGO"
                </a>
            </div>
            <ul>
                <li>
                    <a href="/">
                        "Home"
                    </a>
                </li>
                <li>
                    <a href="/code">
                        "Code"
                    </a>
                </li>
                <li>
                    <a href="/last-years">
                        "Last Years"
                    </a>
                </li>
            </ul>
            <div class="profile">
                "Profile"
            </div>
        </nav>
    }
}

#[component]
fn Sidebar(cx: Scope) -> impl IntoView {
    let query = use_query_map(cx);
    let user = move || query.with(|params| params.get("user").cloned().unwrap_or_default());

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

    let (users, _) = create_signal(cx, names);

    view! { cx,
        <section class="sidebar">
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
                <For each=users key=|name| name.to_owned() view=move|cx, name| {
                    let is_active = move || name == user();
                    view! {cx,
                        <li>
                            <a href="?user={name}" class:active=is_active>{name}</a>
                        </li>
                    }
                }/>
            </ul>
        </section>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <section>
            <h1>"Not Found"</h1>
        </section>
    }
}

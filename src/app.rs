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

        // content for this welcome page
        <Router>
            <Navigation />
            <main>
                <Routes>
                    <Route path="" view=CodeView/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn CodeView(cx: Scope) -> impl IntoView {
    view! { cx,
        <Sidebar />
        <section class="code-overview">
            <div class="code-snippet">
                Code Snippet
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
                "LOGO"
            </div>
            <ul>
                <li>
                    <a href="#">
                        "Overview"
                    </a>
                </li>
                <li>
                    <a href="#">
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
                <li class="active"><a href="#">H1ghBre4k3r</a></li>
                <li><a href="#">H1ghBre4k3r</a></li>
                <li><a href="#">H1ghBre4k3r</a></li>
                <li><a href="#">H1ghBre4k3r</a></li>
                <li><a href="#">H1ghBre4k3r</a></li>
                <li><a href="#">H1ghBre4k3r</a></li>
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

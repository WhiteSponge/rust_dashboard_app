pub mod components;
pub mod db;
pub mod errors;
pub mod models;
pub mod pages;
pub mod server_functions;
use pages::{HomePage, TeamPage};

use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    let script_url = "https://cdn.jsdelivr.net/npm/echarts@5.4.2/dist/echarts.min.js".to_string();
    let script_gl_url =
        "https://cdn.jsdelivr.net/npm/echarts-gl@2.0.9/dist/echarts-gl.min.js".to_string();

    let script_url_team = script_url.clone();
    let script_gl_url_team = script_gl_url.clone();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dashboard-app.css"/>
        <link data-trunk rel="tailwind-css" href="/style/input.css" />

        // sets the document title
        <Title text="Full-Stack Dashboard App"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=move || {
                        view! {
                            <HomePage />

                            <script src=&script_gl_url></script>
                            <script src=&script_url></script>
                        }
                    }/>
                    <Route path="/team" view=move || {
                        view! {
                            <TeamPage />
                            <script src=&script_gl_url_team></script>
                            <script src=&script_url_team></script>
                        }
                    }/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
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
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}

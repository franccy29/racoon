use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::features::connect::connect::Connect;
use crate::features::portefolio::portefolio::Portefolio;
use crate::features::password::password::Password;

#[component]
pub fn App() -> impl IntoView {
    logging::log!("where do I run?");
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/fm.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Connect />
                <Routes>
                    <Route path="" view=Portefolio />
                    <Route path="/password" view=Password />
                </Routes>
            </main>
        </Router>
    }
}

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <script src="https://cdn.jsdelivr.net/npm/@tailwindcss/browser@4"></script>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/test-website.css"/>

        // sets the document title
        <Title text="Welcome to my Webiste"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="w-full flex flex-col justify-center items-center">
            <div class="max-w-2xl content-start p-6">
                <div class="p-3 flex flex-col justify-center items-center">
                    <img class="w-70" src="./striezi.jpeg"/>
                </div>
                <h1 class="text-4xl text-left mb-3">"Striezi"</h1>
                <h3 class="text-2xl text-left mb-2">"Strie|zi"</h3>
                <p class="text-lg text-left">
                    "〈m.9; bayr.–österr.〉"
                    <span class="italic">"liebenswerter Lausbub, harmloser Gauner;"</span>
                    "<auch>"
                </p>
                <p class="text-lg text-left">
                    "〈österr.〉"
                    <span class="italic">"Strizzi"</span>
                </p>
            </div>
        </div>
    }
}

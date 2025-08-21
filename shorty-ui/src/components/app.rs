use leptos::prelude::*;
use super::url_shortner::UrlShortner;

#[component]
pub fn App() -> impl IntoView {
    console_log::init_with_level(log::Level::Debug)
        .expect("console_log failed to initialize");
    view! {
        <header>
            <span class="logo-text not-selectable">
                <span class="logo-text-capital">S</span>horty
            </span>
        </header>
        <main>
            <UrlShortner />
        </main>
    }
}
use crate::components::ThemeToggle;
use leptos::{children::Children, prelude::*};
use leptos_router::components::A;

#[component]
pub fn SiteShell(children: Children) -> impl IntoView {
    view! {
        <a class="skip-link" href="#content">
            "Skip to content"
        </a>

        <div class="site-shell">
            <nav class="site-nav" aria-label="Primary">
                <A href="/" exact=true attr:class="nav-brand">
                    "ZKC"
                </A>

                <div class="nav-actions">
                    <ThemeToggle />
                </div>
            </nav>

            <main id="content" class="site-main">
                {children()}
            </main>
        </div>
    }
}

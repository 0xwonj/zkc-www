use crate::{
    components::SiteShell,
    pages::{HomePage, NotFoundPage, ProtocolIrPage},
};
use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <MetaTags />
        <Meta
            name="description"
            content="Research notes on zero-knowledge compilers, centered on Protocol IR."
        />

        <Router>
            <SiteShell>
                <Routes fallback=|| view! { <NotFoundPage /> }>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/protocol-ir") view=ProtocolIrPage />
                    // Keep the legacy archive path as a compatibility alias.
                    <Route path=path!("/archive") view=ProtocolIrPage />
                </Routes>
            </SiteShell>
        </Router>
    }
}

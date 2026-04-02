use leptos::prelude::*;

#[component]
pub fn PageHeader(title: &'static str, subtitle: &'static str) -> impl IntoView {
    view! {
        <header class="paper-header">
            <h1 class="paper-project">{title}</h1>
            <p class="paper-title">{subtitle}</p>
        </header>
    }
}

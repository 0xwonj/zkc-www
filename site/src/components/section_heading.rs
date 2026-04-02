use leptos::prelude::*;

#[component]
pub fn SectionHeading(title: &'static str) -> impl IntoView {
    view! {
        <h2 class="paper-heading">
            <span class="paper-heading-marker">"§"</span>
            {format!(" {}", title)}
        </h2>
    }
}

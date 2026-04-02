use leptos::{ev::MouseEvent, prelude::*};

const THEME_STORAGE_KEY: &str = "zkc-theme-preference";

fn current_theme() -> &'static str {
    let Some(window) = web_sys::window() else {
        return "light";
    };
    let Some(document) = window.document() else {
        return "light";
    };
    let Some(root) = document.document_element() else {
        return "light";
    };

    match root.get_attribute("data-theme").as_deref() {
        Some("dark") => "dark",
        _ => "light",
    }
}

fn apply_theme(theme: &str) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(root) = document.document_element() else {
        return;
    };

    let _ = root.set_attribute("data-theme", theme);

    if let Ok(Some(meta)) = document.query_selector(r#"meta[name="theme-color"]"#) {
        let color = if theme == "dark" {
            "#0a0a0c"
        } else {
            "#fafafa"
        };
        let _ = meta.set_attribute("content", color);
    }

    if let Ok(Some(storage)) = window.local_storage() {
        let _ = storage.set_item(THEME_STORAGE_KEY, theme);
    }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = RwSignal::new(current_theme());

    let on_click = move |_: MouseEvent| {
        let next = if theme.get() == "dark" {
            "light"
        } else {
            "dark"
        };
        apply_theme(next);
        theme.set(next);
    };

    view! {
        <button
            class="theme-toggle"
            type="button"
            aria-label=move || {
                if theme.get() == "dark" {
                    "Switch to light theme"
                } else {
                    "Switch to dark theme"
                }
            }
            aria-pressed=move || theme.get() == "dark"
            title=move || {
                if theme.get() == "dark" {
                    "Dark theme"
                } else {
                    "Light theme"
                }
            }
            on:click=on_click
        >
            {move || if theme.get() == "dark" { "🌙" } else { "☀️" }}
        </button>
    }
}

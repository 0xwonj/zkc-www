mod app;
mod components;
mod data;
mod pages;

use leptos::{mount::mount_to_body, prelude::*};

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <app::App /> });
}

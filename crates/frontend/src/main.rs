#![allow(non_snake_case, dead_code)]
mod assets;
mod components;
mod constants;
mod error;
mod hooks;
mod models;
mod pages;
mod routes;
mod services;

use dioxus::prelude::*;

#[cfg(debug_assertions)]
use tracing::Level;

fn main() {
    init_client_logger();
    services::panic_on_error();
    launch(App);
}

fn App() -> Element {
    hooks::init_theme();
    rsx! {
        div { class: "bg-light text-slate-950 dark:text-slate-50 dark:bg-dark",
            Router::<routes::Routes> {}
        }
    }
}

#[cfg(debug_assertions)]
fn init_client_logger() {
    dioxus_logger::init(Level::DEBUG).expect("failed to init logger");
}

#[cfg(not(debug_assertions))]
fn init_client_logger() {}

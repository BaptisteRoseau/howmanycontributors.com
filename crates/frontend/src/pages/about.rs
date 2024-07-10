use crate::components::{Footer, Header};

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        body { class: "flex flex-col min-h-screen justify-between",
            Header {}
            Footer {}
        }
    }
}

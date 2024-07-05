use crate::components::{Footer, Header};

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        Header {}
        Footer {}
    }
}

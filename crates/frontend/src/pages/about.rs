use crate::components::{Footer, Hero};

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        // Header { theme }
        Hero {},
        Footer {}
    }
}

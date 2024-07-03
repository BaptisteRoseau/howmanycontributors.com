use crate::{
    components::{Footer, Header},
    hooks::use_theme,
};

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let theme = use_theme();
    rsx! {
        Header { theme }
        Footer {}
    }
}

use crate::{
    components::{Footer, Header, Hero, MainSearch},
    hooks::use_theme,
};

use dioxus::prelude::*;

#[component]
pub fn Home(url: Option<String>) -> Element {
    let theme = use_theme();
    rsx! {
        Header { theme }
        Hero {}
        MainSearch { url }
        Footer {}
    }
}

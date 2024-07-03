use crate::{
    components::{Footer, Header, Hero, MainSearch},
    hooks::use_theme,
};

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let theme = use_theme();
    rsx! {
        Header { theme }
        Hero {},
        MainSearch {},
        div { class: "h-80%" },
        Footer {}
    }
}

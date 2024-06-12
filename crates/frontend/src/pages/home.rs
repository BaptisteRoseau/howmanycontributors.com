use crate::{
    assets::{MoonIcon, PcIcon, SunIcon},
    components::{Footer, Header, Hero},
    hooks::use_theme,
};

use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let theme = use_theme();
    let selected = use_signal(|| true);
    rsx! {
        Header { theme }
        Hero {},
        div { class: "container", SunIcon {selected}, MoonIcon {selected}, PcIcon {selected} },
        Footer {}
    }
}

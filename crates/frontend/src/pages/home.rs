use crate::components::{Footer, Header, Hero, MainSearch};

use dioxus::prelude::*;

#[component]
pub fn Home(url: Option<String>) -> Element {
    rsx! {
        Header {  }
        Hero {}
        MainSearch { url }
        Footer {}
    }
}

use crate::components::{Footer, Header, Hero, MainSearch};

use dioxus::prelude::*;

#[component]
pub fn Home(url: Option<String>) -> Element {
    rsx! {
        body {
            class: "flex flex-col min-h-screen justify-between",
            body {
                Header {  }
                Hero {}
                MainSearch { url }
            }
            Footer {}
        }
    }
}

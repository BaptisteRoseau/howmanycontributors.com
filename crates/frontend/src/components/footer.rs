use crate::{assets::LogoText, routes::Routes};

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "container",
            p { class: "my-4 text-center text-sm text-gray-500",
                "Copyright @ 2024. All rights reserved."
            }
        }
    }
}

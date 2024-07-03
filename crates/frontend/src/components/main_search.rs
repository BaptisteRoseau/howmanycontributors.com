use crate::assets::Logo;

use dioxus::prelude::*;

#[component]
pub fn MainSearch() -> Element {
    rsx! {
        section { class: "container",
            div { class: "py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12",
                input {
                    "type": "search",
                    class: "border rounded-lg w-full",
                    id: "main_search",
                    // placeholder: "Your GitHub's repository URL",
                }
            }
        }
    }
}

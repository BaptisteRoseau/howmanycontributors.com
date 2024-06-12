use crate::assets::LogoText;

use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        footer { class: "container",
            div { class: "mx-auto max-w-screen-xl px-4 py-8 sm:px-6 lg:px-8",
                div { class: "sm:flex sm:items-center sm:justify-between",
                    div { class: "flex justify-center text-blue-500 sm:justify-start",
                        LogoText {}
                    }
                    p { class: "mt-4 text-center text-sm text-gray-500 lg:mt-0 lg:text-right",
                        "Copyright @ 2024. All rights reserved."
                    }
                }
            }
        }
    }
}

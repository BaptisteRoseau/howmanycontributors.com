use crate::components::{Footer, Header};
use crate::hooks::use_theme;
use crate::routes::Routes;

use dioxus::prelude::*;

#[component]
pub fn NotFound(segments: Vec<String>) -> Element {
    let theme = use_theme();
    rsx! {
        Header { theme },
        div { class: "grid h-screen place-content-center px-4",
            div { class: "text-center",
                h1 { class: "text-9xl font-black text-gray-300 dark:text-gray-700",
                    "404"
                }
                p { class: "text-2xl font-bold tracking-tight text-gray-700 dark:text-gray-50 sm:text-4xl",
                    "Uh-oh!"
                }
                p { class: "mt-4 text-gray-500", "We can't find that page." }
                Link {
                    to: Routes::Home {},
                    class: "mt-6 inline-block rounded bg-indigo-600 px-5 py-3 text-sm font-medium text-white hover:bg-indigo-700 focus:outline-none focus:ring",
                    "Go Back Home"
                }
            }
        }
        Footer {}
    }
}

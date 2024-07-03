use crate::assets::Logo;

use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "container",
            div { class: "py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12",
                h1 { class: "mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white",
                    "How Many Contributors ?"
                }
                h2 { class: "mb-8 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 xl:px-48 dark:text-gray-400",
                    "Ever wondered how many contributors a project "
                    i {"really"}
                    " has ? Let's find out !"
                }
                p {
                    "When building a new project, we rely on software that relies on software that relies on software..."
                    br {}
                    "Thousands of developpers enabled us to build our project."
                }
            }
        }
    }
}

use crate::assets::Logo;

use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "container",
            div { class: "pt-8 pb-4 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12",
                h1 { class: "mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl lg:text-6xl",
                    "How Many Contributors ?"
                }
                h2 { class: "mb-8 text-lg font-normal lg:text-xl sm:px-16 xl:px-48",
                    "Ever wondered how many contributors a project "
                    i {"really"}
                    " has ? Let's find out !"
                }
                p {
                    "When building a new project, we rely on software that relies on software that relies on software..."
                    "Thousands of developers enabled us to build it!"
                    br {}
                    "This is you chance to know how many developers made your work possible."
                }
            }
        }
    }
}

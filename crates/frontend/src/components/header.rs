use crate::assets::{Logo, Menu};
use crate::components::ThemeSwitcher;
use crate::hooks::ThemeHandler;
use crate::routes::Routes;

use dioxus::prelude::*;

#[component]
pub fn Header(theme: Signal<ThemeHandler>) -> Element {
    // TODO: Display the user's name and image in the header
    rsx! {
        header { class: "container",
            div { class: "mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    div { class: "md:flex md:items-center md:gap-12",
                        a { href: "#", class: "block text-teal-600",
                            span { class: "sr-only", "Home" }
                            Logo {}
                        }
                    }
                    div {
                        nav { "aria-label": "Global",
                            ul { class: "flex items-center gap-6 text-sm",
                                li {
                                    a {
                                        href: "#",
                                        class: "text-gray-500 transition hover:text-gray-500/75",
                                        "About"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

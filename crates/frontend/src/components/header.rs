use crate::assets::{Logo, Menu};
use crate::components::ThemeSwitcher;
use crate::hooks::ThemeHandler;
use crate::routes::Routes;

use dioxus::prelude::*;

#[component]
pub fn Header(theme: Signal<ThemeHandler>) -> Element {
    rsx! {
        header { class: "container",
            div { class: "mx-auto max-w-screen-xl px-4 sm:px-6 lg:px-8",
                div { class: "flex h-16 items-center justify-between",
                    div { class: "md:flex md:items-center md:gap-12",
                        Link { class: "block text-teal-600", to: Routes::Home {},
                            span { class: "sr-only", "Home" }
                            Logo {}
                        }
                    }
                    div {
                        class: "flex items-center gap-6 text-sm",
                        nav { "aria-label": "Global",
                            ul { class: "flex items-center gap-6 text-sm",
                                li {
                                    Link {
                                        to: Routes::Home {},
                                        class: "text-gray-500 transition hover:text-gray-500/75",
                                        "Home"
                                    }
                                }
                                li {
                                    Link {
                                        to: Routes::About {},
                                        class: "text-gray-500 transition hover:text-gray-500/75",
                                        "About"
                                    }
                                }
                                // li {
                                //     Link {
                                //         to: Routes::Leaderboard {},
                                //         class: "text-gray-500 transition hover:text-gray-500/75",
                                //         "Leaderboard"
                                //     }
                                // }
                            }
                        }
                        ThemeSwitcher { theme },
                    }
                }
            }
        }
    }
}

use crate::assets::{Logo, Menu};
use crate::components::ThemeSwitcher;
use crate::hooks::{use_theme, ThemeHandler};
use crate::routes::Routes;

use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let theme = use_theme();
    rsx! {
        header { class: "container",
            div { class: "mx-auto my-2 flex justify-center items-center gap-6",
                nav { "aria-label": "Global",
                    ul { class: "flex items-center gap-6 text-sm flex-1 md:text-md 2xl:text-lg",
                        li {
                            Link {
                                to: Routes::Home {},
                                class: "text-gray-700 transition-color hover:text-gray-700/75 dark:text-gray-300 dark:hover:text-gray-300/75",
                                "Home"
                            }
                        }
                        li {
                            Link {
                                to: Routes::Leaderboard {},
                                class: "text-gray-700 transition-color hover:text-gray-700/75 dark:text-gray-300 dark:hover:text-gray-300/75",
                                "Leaderboard"
                            }
                        }
                        li {
                            Link {
                                to: Routes::About {},
                                class: "text-gray-700 transition-color hover:text-gray-700/75 dark:text-gray-300 dark:hover:text-gray-300/75",
                                "About"
                            }
                        }
                    }
                }
                div {class: "border-l border-slate-200 dark:border-gray-500 h-6"}
                ThemeSwitcher { theme }
            }
        }
    }
}

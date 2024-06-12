use crate::assets::Logo;

use dioxus::prelude::*;

#[component]
pub fn Hero() -> Element {
    rsx! {
        section { class: "container",
            div { class: "py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12",
                Logo {}
                h1 { class: "mb-4 text-4xl font-extrabold tracking-tight leading-none text-gray-900 md:text-5xl lg:text-6xl dark:text-white",
                    "Learn easily with hmc"
                }
                p { class: "mb-8 text-lg font-normal text-gray-500 lg:text-xl sm:px-16 xl:px-48 dark:text-gray-400",
                    "Here at hmc, we provide you with the best learning experience. Our platform is designed to help you learn easily and effectively with powerful flashcards."
                }
                div { class: "flex flex-col mb-8 lg:mb-16 space-y-4 sm:flex-row sm:justify-center sm:space-y-0 sm:space-x-4",
                    a {
                        href: "#",
                        class: "inline-flex justify-center items-center py-3 px-5 text-base font-medium text-center text-white rounded-lg bg-primary-700 hover:bg-primary-800 focus:ring-4 focus:ring-primary-300 dark:focus:ring-primary-900",
                        "Learn more"
                        svg {
                            "viewBox": "0 0 20 20",
                            "xmlns": "http://www.w3.org/2000/svg",
                            "fill": "currentColor",
                            class: "ml-2 -mr-1 w-5 h-5",
                            path {
                                "clip-rule": "evenodd",
                                "fill-rule": "evenodd",
                                "d": "M10.293 3.293a1 1 0 011.414 0l6 6a1 1 0 010 1.414l-6 6a1 1 0 01-1.414-1.414L14.586 11H3a1 1 0 110-2h11.586l-4.293-4.293a1 1 0 010-1.414z"
                            }
                        }
                    }
                }
            }
        }
    }
}

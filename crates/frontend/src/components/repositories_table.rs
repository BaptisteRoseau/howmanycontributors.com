use dioxus::prelude::*;

#[component]
pub fn RepositoriesTable(repositories: Signal<Vec<(String, usize)>>) -> Element {
    rsx! {
        table { class: "table-auto mx-auto text-[0.75em] sm:text-sm md:text-md font-light text-surface dark:text-white max-w-[340px] sm:max-w-screen-sm md:max-w-screen-md lg:max-w-screen-lg",
            thead { class: "border-b border-neutral-200 font-medium dark:border-white/10",
                tr {
                    th { scope: "col", class: "text-center px-6 py-4", "Rank" }
                    th { scope: "col", class: "text-left px-6 py-4 break-all", "Repository" }
                    th { scope: "col", class: "text-center px-6 py-4", "Contributors" }
                }
            }
            tbody { class: "text-center text-0.5em sm:text-sm md:text-md",
                for (idx , (repository , contributors)) in repositories.read().iter().enumerate() {
                    tr { key: "{repository}", class: "border-b border-neutral-200 transition duration-300 ease-in-out hover:bg-neutral-200 dark:border-white/10 dark:hover:bg-neutral-600",
                        td { class: "text-center px-6 py-2", "#{idx+1}" }
                        td { class: "text-left px-6 py-2 break-all",
                            a {
                                href: "https://github.com/{repository}",
                                target: "_blank",
                                class: "hover:text-sky-500",
                                "{repository}"
                            }
                        }
                        td { class: "text-center px-6 py-2", "{contributors}" }
                    }
                }
            }
        }
    }
}

use crate::components::{Footer, Header};

use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let questions_answers: Vec<(&str, &str)> = vec![
        ("Are the contribution counts exact ?", "No. They are the moment they are fetched, however to avoid flooding GitHub the results are cached for several days so they may not be up to date. Moreover, some contributors may have contributed to several dependencies and be counted multiple times. This website does not try to provide an exact number of contributors, but an order of magnitude of how many people made your work possible."),
        ("How are the number of contributors fetched ?", "They are extracted from the GitHub repository's main page and its \"Insights\" -> \"Dependency graph\" page. We do not mean any harm to GitHub APIs so the results are cached several days and dependency fetching is delayed, this is why you may often see the number of contributors and dependencies slowly growing."),
        ("Who made this website ?", "👋 Hi ! I am Baptiste Roseau, a backend engineer currently learning frontend and devops skills. This website was an idea to practice Rust's WebAssembly, WebSocket and some juicy DevOps tools on its backend. Still some work to do on the frontend but I will improve it over time."),
        ("What data do you save ?", "We only save the dependencies and number of contributors of each repository searched on this website, as well as some volumetric data such as the number of requests or messages sent via websocket. Your theme preference (light/dark/system) is also saved in your browser."),
        ("How does the leaderboard work ?", "Each time a repository or its dependency is searched on this website, its number of contributors is saved into the leaderboard. Only the top 500 remains, so if you want to see your favorite projects in it, go search them on the home page!"),
    ];

    rsx! {
        body { class: "flex flex-col min-h-screen justify-between",
            body {
                Header {}
                section { class: "container",
                    div { class: "py-8 px-4 mx-auto text-center lg:py-16 lg:px-12",
                        h1 { class: "mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl lg:text-6xl",
                            "About This Website"
                        }
                        p { class: "w-full mx-auto mb-2 text-md font-normal text-center lg:text-lg sm:px-16 xl:px-48",
                            "Hi ! 👋"
                            br {}
                            "This website aims to remind us that none of our work would have been possible without the combined effort of thousands of developers."
                            br {}
                            "It "
                            i { "recursively" }
                            " finds the number of contributors of a GitHub project and all its dependencies."
                        }
                    }
                }
                section { class: "container",
                    ul { class: "flex flex-col",
                        for (question , answer) in questions_answers.iter() {
                            li { class: "bg-white dark:bg-slate-200 text-black my-1 shadow-lg",
                                details {
                                    summary { class: "border-l-3 border-pri-600 font-semibold p-3 cursor-pointer",
                                        "{question}"
                                    }
                                    div { class: "border-l-3 border-pri-600 overflow-hidden duration-500 transition-all",
                                        p { class: "p-3 text-gray-900", "{answer}" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Footer {}
        }
    }
}

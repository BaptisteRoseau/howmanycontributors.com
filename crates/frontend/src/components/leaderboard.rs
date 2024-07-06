use crate::services::get_leaderboard;
use crate::{assets::Logo, error::Error};
use crate::{assets::LogoText, routes::Routes};
use tracing::{debug, error, info};

use dioxus::prelude::*;

#[component]
pub fn Leaderboard() -> Element {
    let mut error_msg = use_signal(|| "");
    let mut repositories: Signal<Vec<(String, usize)>> = use_signal(Vec::new);

    let fetch = move || {
        spawn(async move {
            match get_leaderboard().await {
                Ok(mut leaderboard) => {
                    while let Some(item) = leaderboard.pop() {
                        repositories.write().push((item.0, item.1 as usize));
                        repositories.write().sort_by(|a, b| b.1.cmp(&a.1));
                    }
                }
                Err(e) => {
                    error!("Error Fetching dependencies: {:#?}", e);
                    error_msg.set("Whoops, something went wrong!");
                }
            };
        });
    };
    let onclick = move |_| {
        fetch();
    };

    // Fetch results when going onto the page
    use_effect(fetch);

    rsx! {
        section { class: "container",
            div { class: "py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12",
                h1 { class: "mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl lg:text-6xl",
                    "Most Contributors"
                }
                h2 { class: "mb-8 text-lg font-normal lg:text-xl sm:px-16 xl:px-48",
                    "What are the projects with the most contributors ?"
                }
                if !error_msg.read().is_empty() {
                    p { class: "mx-auto border-l-red-500 border-l-4 bg-opacity-80 bg-slate-200 text-red-700 text-center text-lg py-2 w-full dark:bg-slate-800",
                        "{error_msg}"
                    }
                }
                button {
                    class: "mx-auto cursor-pointer bg-black py-2 px-4 rounded-lg text-white border border-white mt-4 dark:border-black dark:text-black dark:bg-white  disabled:cursor-not-allowed disabled:bg-gray-300 disabled:text-gray-600",
                    "type": "submit",
                    onclick: onclick,
                    "Refresh"
                }
                table { class: "table-auto text-center min-w-full mx-auto text-left text-sm font-light text-surface dark:text-white",
                    thead { class: "text-center border-b border-neutral-200 font-medium dark:border-white/10",
                        tr {
                            th { scope: "col", class: "px-6 py-4", "Repository" }
                            th { scope: "col", class: "px-6 py-4", "Contributors" }
                        }
                    }
                    tbody { class: "text-center",
                        for (repository , contributors) in repositories.read().iter() {
                            tr { key: "{repository}", class: "border-b border-neutral-200 transition duration-300 ease-in-out hover:bg-neutral-100 dark:border-white/10 dark:hover:bg-neutral-600",
                                td { class: "text-right whitespace-nowrap px-6 py-4",
                                    a {
                                        class: "hover:text-pri-500",
                                        href: "https://github.com/{repository}",
                                        "{repository}"
                                    }
                                }
                                td { class: "whitespace-nowrap px-6 py-4", "{contributors}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

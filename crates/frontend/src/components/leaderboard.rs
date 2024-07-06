use std::collections::BTreeMap;

use crate::services::get_leaderboard;
use crate::{assets::Logo, error::Error};
use crate::{assets::LogoText, routes::Routes};
use tracing::{debug, error, info};

use dioxus::prelude::*;

#[component]
pub fn Leaderboard() -> Element {
    let mut error_msg = use_signal(|| "");
    let mut repositories: Signal<BTreeMap<usize, String>> = use_signal(|| BTreeMap::new());

    let fetch = move || {
        spawn(async move {
            match get_leaderboard().await {
                Ok(mut leaderboard) => {
                    while let Some(item) = leaderboard.pop() {
                        repositories.write().insert(item.1 as usize, item.0);
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
            h2 { class: "text-center w-full mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-4xl lg:text-5xl dark:text-white",
                "Most Contributors"
            }
            h3 { class: "mx-auto text-center w-full mb-4 text-xl font-extrabold leading-none tracking-tight text-gray-900 md:text-4xl lg:text-5xl dark:text-white",
                "What are the projects with the most contributors ?"
            }
            if !error_msg.read().is_empty() {
                p { class: "mx-auto bg-opacity-80 border-red border-l-2 bg-slate-600 transition-transform duration-500 text-red text-center text-lg py-2 w-full",
                    "{error_msg}"
                }
            }
            button {
                class: "mx-auto cursor-pointer bg-pri-500 py-2 px-4 rounded-lg text-white border mt-4 disabled:bg-gray-300 disabled:text-gray-600",
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
                for (contributors, repository) in repositories.read().iter().rev() {
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

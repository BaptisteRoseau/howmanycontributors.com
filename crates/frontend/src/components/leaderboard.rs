use crate::components::RepositoriesTable;
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
                    repositories.write().clear();
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

    // Fetch results when loading the page
    use_effect(fetch);

    rsx! {
        section { class: "container py-8 px-4 mx-auto text-center lg:py-16 lg:px-12",
            h1 { class: "mb-4 text-4xl font-extrabold tracking-tight leading-none md:text-5xl lg:text-6xl",
                "Most Contributors"
            }
            h2 { class: "mb-2 text-lg font-normal lg:text-xl sm:px-16 xl:px-48",
                "Top 500 GitHub repositories with the most contributors!"
            }
            p { class: "mb-8 text-sm font-normal text-slate-400 dark:text-slate-600 lg:text-md sm:px-16 xl:px-48",
                "*The repositories listed here come from previous searches."
            }
            if !error_msg.read().is_empty() {
                p { class: "mb-4 mx-auto border-l-red-500 border-l-4 rounded-r-full bg-opacity-60 bg-slate-200 text-red-700 text-center text-lg py-2 w-full dark:bg-slate-900",
                    "{error_msg}"
                }
            }
            RepositoriesTable { repositories }
        }
    }
}

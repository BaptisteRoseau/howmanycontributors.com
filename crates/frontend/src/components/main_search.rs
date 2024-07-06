use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::ops::{Add, Deref, DerefMut};

use crate::models::ContributorsChunk;
use crate::services::get_dependencies;
use crate::{assets::Logo, error::Error};

use dioxus::prelude::*;
use lazy_static::lazy_static;
use regex::Regex;
use tracing::{debug, error, info};

lazy_static! {
    static ref LINK_PATTERN: Regex =
        Regex::new(r#"^https?://github.com/([a-zA-Z0-9_\.-]{1,35})/([a-zA-Z0-9_\.-]{1,101})/?$"#)
            .unwrap();
}

fn is_link_valid(link: &str) -> bool {
    LINK_PATTERN.captures(link.trim()).is_some()
}

//TODO: Keyboard shortcuts
// - Enter: Search
// - /: Focus on Search

#[component]
pub fn MainSearch(url: Option<String>) -> Element {
    let mut button_disabled = use_signal(|| false);
    let mut running = use_signal(|| false);
    let mut url = use_signal(|| url.unwrap_or("".to_string()));
    let mut error_msg = use_signal(|| "");
    let mut total_contributors = use_signal(|| 0_usize);
    let mut repositories: Signal<BTreeMap<usize, String>> = use_signal(BTreeMap::new);

    //TODO: A possibility to cancel the search

    let onclick = move |_| {
        debug!("Button pressed with: {}", url.read());
        if url.read().is_empty() {
            error_msg.set("Please fill this field.");
            return;
        };
        if !is_link_valid(url.read().as_str()) {
            error_msg.set("Please provide a valid GitHub repository link.");
            return;
        };

        spawn(async move {
            error_msg.set("");
            total_contributors.set(0);
            repositories.write().deref_mut().clear();
            button_disabled.set(true);
            running.set(true);

            let u = url.read();
            let u = u.as_str();
            let handle_chunk = move |chunk: ContributorsChunk| {
                total_contributors += chunk.contributors;
                repositories.write().insert(chunk.contributors, chunk.path);
            };
            if let Err(e) = get_dependencies(u, handle_chunk) {
                error!("Error Fetching dependencies: {:#?}", e);
                match e {
                    Error::NotFound => {
                        error_msg.set("This repository does not exist.");
                    }
                    _ => {
                        error_msg.set("Whoops, something went wrong!");
                    }
                }
            };
            button_disabled.set(false);
            running.set(false);
        });
    };

    let onstop = move |_| {
        debug!("Cancel button pressed");
    };

    rsx! {
        section { class: "container",
            div { class: "py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12",
                if !error_msg.read().is_empty() {
                    h3 { class: "bg-opacity-80 border-red border-l-2 bg-slate-600 transition-transform duration-500 text-red text-center text-lg py-2 w-full",
                        "{error_msg}"
                    }
                }
                input {
                    "type": "search",
                    class: "p-2 bg-stone-300 border border-pri-300 rounded-lg w-full text-black",
                    id: "main_search",
                    placeholder: "https://github.com/owner/repository",
                    value: "{url}",
                    oninput: move |event| url.set(event.value()),
                    maxlength: 300
                }
                div { class: "flex justify-center gap-2",
                    button {
                        class: "cursor-pointer bg-pri-500 py-2 px-4 rounded-lg text-white border mt-4 disabled:bg-gray-300 disabled:text-gray-600",
                        "type": "submit",
                        onclick: onclick,
                        disabled: button_disabled,
                        "Let's find out!"
                    }
                    if *running.read() {
                        button {
                            class: "cursor-pointer bg-red py-2 px-4 rounded-lg text-white border mt-4 disabled:bg-gray-300 disabled:text-gray-600",
                            "type": "submit",
                            onclick: onstop,
                            disabled: !*running.read(),
                            "Stop"
                        }
                    }
                }
                if *total_contributors.read() > 0 {
                    div { class: "text-center text-3xl w-full",
                        "Found "
                        strong { class: "text-9xl", "{total_contributors}" }
                        " contributors from "
                        strong { class: "text-7xl", "{repositories.read().len()}" }
                        " total dependencies !"
                    }
                }
            }
        }
        if !repositories.read().is_empty() {
            section { class: "container",
                h2 { class: "text-center w-full mb-4 text-3xl font-extrabold leading-none tracking-tight text-gray-900 md:text-4xl lg:text-5xl dark:text-white",
                    "Dependencies Contributors"
                }
                table { class: "table-auto text-center min-w-full mx-auto text-left text-sm font-light text-surface dark:text-white",
                    thead { class: "text-center border-b border-neutral-200 font-medium dark:border-white/10",
                        tr {
                            th { scope: "col", class: "px-6 py-4", "Repository" }
                            th { scope: "col", class: "px-6 py-4", "Contributors" }
                        }
                    }
                    tbody { class: "text-center",
                        for (contributors , repository) in repositories.read().iter().rev() {
                            tr { key: "{repository}", class: "border-b border-neutral-200 transition duration-300 ease-in-out hover:bg-neutral-100 dark:border-white/10 dark:hover:bg-neutral-600",
                                td { class: "whitespace-nowrap px-6 py-4", "{repository}" }
                                td { class: "whitespace-nowrap px-6 py-4", "{contributors}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

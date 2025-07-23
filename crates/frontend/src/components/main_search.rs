use std::borrow::BorrowMut;
use std::collections::{BTreeMap, HashMap};
use std::ops::{Add, Deref, DerefMut};

use crate::components::RepositoriesTable;
use crate::models::ContributorsChunk;
use crate::services::{ServiceWebsocket, get_dependencies};
use crate::{assets::Logo, error::Error};

use dioxus::prelude::*;
use gloo::timers::future::sleep;
use lazy_static::lazy_static;
use regex::Regex;
use std::time::Duration;
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
    let mut should_stop = use_signal(|| false);
    let mut url = use_signal(|| url.unwrap_or("".to_string()));
    let mut error_msg = use_signal(|| "");
    let mut total_contributors = use_signal(|| 0_usize);
    let mut repositories: Signal<Vec<(String, usize)>> = use_signal(Vec::new);

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
                repositories.write().push((chunk.path, chunk.contributors));
                repositories.write().deref_mut().sort_by(|a, b| {
                    if b.1 != a.1 {
                        b.1.cmp(&a.1)
                    } else {
                        b.0.cmp(&a.0)
                    }
                });
            };
            match get_dependencies(u, handle_chunk) {
                Ok(mut ws) => {
                    let _ = ws;
                    debug!("Awaiting stop");
                    while ws.is_open() && !*should_stop.read() {
                        sleep(Duration::from_millis(200)).await;
                    }
                    ws.close();
                    should_stop.set(false);
                }
                Err(e) => {
                    error!("Error Fetching dependencies: {:#?}", e);
                    match e {
                        Error::NotFound => {
                            error_msg.set("This repository does not exist.");
                        }
                        _ => {
                            error_msg.set("Whoops, something went wrong!");
                        }
                    }
                }
            };
            button_disabled.set(false);
            running.set(false);
        });
    };

    let onstop = move |_| {
        debug!("Cancel button pressed");
        should_stop.set(true);
    };

    rsx! {
        section { class: "container",
            div { class: "px-4 mx-auto max-w-screen-xl text-center lg:px-12",
                if !error_msg.read().is_empty() {
                    p { class: "mb-4 mx-auto border-l-red-500 border-l-4 rounded-r-full bg-opacity-60 bg-slate-200 text-red-700 text-center text-lg py-2 w-full dark:bg-slate-900",
                        "{error_msg}"
                    }
                }
                input {
                    "type": "search",
                    class: " p-2 border border-slate-500 rounded-lg w-full bg-white text-black",
                    id: "main_search",
                    placeholder: "https://github.com/owner/repository",
                    value: "{url}",
                    oninput: move |event| url.set(event.value()),
                    maxlength: 300
                }
                div { class: "flex justify-center gap-2",
                    button {
                        class: "cursor-pointer border border-slate-500 bg-slate-300 py-2 px-4 rounded-lg text-black mt-4 hover:bg-slate-400 disabled:bg-gray-300 disabled:text-gray-600 disabled:cursor-not-allowed",
                        "type": "submit",
                        onclick: onclick,
                        disabled: button_disabled,
                        "Let's find out!"
                    }
                    if *running.read() {
                        button {
                            class: "cursor-pointer bg-red-500 py-2 px-4 rounded-lg text-white border mt-4 disabled:bg-gray-300 disabled:text-gray-600",
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
                        strong { class: "text-9xl text-pri-300 dark:text-pri-800", "{total_contributors}" }
                        " contributors from "
                        strong { class: "text-7xl text-sec-300 dark:text-sec-800", "{repositories.read().len()}" }
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
                RepositoriesTable { repositories }
            }
        }
    }
}

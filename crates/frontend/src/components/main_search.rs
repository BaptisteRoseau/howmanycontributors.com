use crate::services::dependencies;
use crate::{assets::Logo, error::Error};

use dioxus::prelude::*;
use tracing::{debug, error, info};

#[component]
pub fn MainSearch(url: Option<String>) -> Element {
    let mut button_disabled = use_signal(|| false);
    let mut running = use_signal(|| false);
    let mut url = use_signal(|| url.unwrap_or("".to_string()));
    let mut error_msg = use_signal(|| "");
    let mut total_contributors = use_signal(|| 0_usize);
    //TODO: Table with hashmap of all results

    let onclick = move |_| {
        //TODO: Clear previous results
        debug!("Button pressed with: {}", url.read());
        if url.read().is_empty() {
            error_msg.set("Please fill this field.");
            return;
        };

        // See https://github.com/tokio-rs/axum/blob/main/examples/websockets/src/client.rs
        // for the websocket

        spawn(async move {
            total_contributors.set(0);
            running.set(true);
            button_disabled.set(true);
            error_msg.set("");
            let u = url.read();
            let u = u.as_str();
            match dependencies(u).await {
                Ok(deps) => {
                    debug!("{:?}", deps)
                }
                Err(e) => {
                    error!("Error Fetching dependencies: {:#?}", e);
                    match e {
                        Error::NotFound => {
                            error_msg.set("This repository does not exist.");
                        }
                        _ => {
                            error_msg.set("Whoops, something went wrong!");
                            total_contributors.set(100);
                        }
                    }
                }
            };
            running.set(false);
            button_disabled.set(false);
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
                button {
                    class: "cursor-pointer bg-pri-500 py-2 px-4 rounded-lg text-white border mt-4 disabled:bg-gray-300 disabled:text-gray-600",
                    "type": "submit",
                    onclick: onclick,
                    disabled: button_disabled,
                    "Let's find out!"
                }
                if *total_contributors.read() > 0 {
                    div {
                        class: "text-center text-3xl w-full",
                        "Found "
                        strong {
                            class: "text-9xl",
                            "{total_contributors}"
                        }
                        " contributors"
                    }
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
        }
    }
}

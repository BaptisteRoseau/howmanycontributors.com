use crate::services::dependencies;
use crate::{assets::Logo, error::Error};

use dioxus::prelude::*;
use tracing::{debug, error, info};

#[component]
pub fn MainSearch(url: Option<String>) -> Element {
    let mut button_disabled = use_signal(|| false);
    let mut url = use_signal(|| url.unwrap_or("".to_string()));
    let mut error_msg = use_signal(|| "");

    let onclick = move |_| {
        button_disabled.set(true);
        error_msg.set("");
        spawn(async move {
            debug!("Button pressed with: {}", url.read());
            let u = url.read();
            let u = u.as_str();
            if u.is_empty() {
                error_msg.set("Please fill this field.");
                button_disabled.set(false);
                return;
            };
            match dependencies(u).await {
                Ok(deps) => {
                    info!("{:?}", deps)
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
        });
    };

    rsx! {
        section { class: "container",
            div { class: "py-8 px-4 mx-auto max-w-screen-xl text-center lg:py-16 lg:px-12",
                if !error_msg.read().is_empty() {
                    h3 { class: "bg-opacity-80 border-red border-l-2 bg-slate-600 transition-transform duration-500 text-red text-center text-lg py-2 w-96",
                        "{error_msg}"
                    }
                }
                input {
                    "type": "search",
                    class: "p-2 border border-pri-300 rounded-lg w-full text-black",
                    id: "main_search",
                    placeholder: "https://github.com/owner/repository",
                    value: "{url}",
                    oninput: move |event| url.set(event.value()),
                    maxlength: 300,
                },
                button {
                    class: "cursor-pointer bg-pri-500 py-2 px-4 rounded-lg text-white border mt-4 disabled:bg-gray-300 disabled:text-gray-600",
                    "type": "submit",
                    onclick: onclick,
                    disabled: button_disabled,
                    "Let's find out!",
                }
            }
        }
    }
}

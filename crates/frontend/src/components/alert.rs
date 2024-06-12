use dioxus::prelude::*;

#[component]
pub fn AlertBannerGreen(title: String, description: String) -> Element {
    rsx! {
        div {
            role: "alert",
            class: "bg-green-100 border-l-4 border-green-500 text-green-700 p-4",
            p { class: "font-bold", "{title}" }
            p { "{description}" }
        }
    }
}

#[component]
pub fn AlertBannerRed(title: String, description: String) -> Element {
    rsx! {
        div {
            role: "alert",
            class: "bg-red-100 border-l-4 border-red-500 text-red-700 p-4",
            p { class: "font-bold", "{title}" }
            p { "{description}" }
        }
    }
}

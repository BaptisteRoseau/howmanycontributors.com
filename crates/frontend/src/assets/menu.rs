use dioxus::prelude::*;

#[component]
pub fn Menu() -> Element {
    rsx! {
        svg {
            "stroke-width": "2",
            "fill": "none",
            "viewBox": "0 0 24 24",
            "stroke": "currentColor",
            class: "h-5 w-5",
            path {
                "d": "M4 6h16M4 12h16M4 18h16",
                "stroke-linecap": "round",
                "stroke-linejoin": "round"
            }
        }
    }
}

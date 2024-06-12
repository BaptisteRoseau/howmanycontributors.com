use dioxus::prelude::*;

#[component]
pub fn DownArrow() -> Element {
    rsx! {
        svg {
            "fill": "none",
            path {
                "strokeWidth": "2",
                "d": "m15 11-3 3-3-3",
                "strokeLinecap": "round",
                "strokeLinejoin": "round",
                "stroke": "currentColor"
            }
        }
    }
}

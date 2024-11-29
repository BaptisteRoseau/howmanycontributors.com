use crate::routes::Routes;

use dioxus::prelude::*;

const BUTTON_THEME: &str = "bg-primary-300 hover:bg-primary-500 focus:hover:bg-primary-700 dark:bg-primary-700 dark:hover:bg-primary-500 dark:focus:hover:bg-primary-300 text-black  dark:text-white font-bold py-[0.5em] px-[1.5em] rounded";


// An example of how to build a component that will
// accept global tags.
// #[derive(Props, Clone, PartialEq)]
// pub struct ButtonProps {
//     pub class: Option<String>,
//     pub children: Element,
//     #[props(extends = GlobalAttributes)]
//     attributes: Vec<Attribute>,
// }

// pub fn Button(props: ButtonProps) -> Element {
//     let ButtonProps {
//         class,
//         attributes,
//         children,
//         ..
//     } = props;

//     let mut class_ = String::from(BUTTON_THEME);
//     if let Some(c) = class {
//         class_.push_str(&c);
//     };
//     let class = class_;

//     rsx! {
//         a {
//             class,
//             ..attributes,
//             {children}
//         }
//     }
// }


#[component]
pub fn LinkButton(to: Routes, label: &'static str) -> Element {
    rsx! {
        Link { class: BUTTON_THEME, to, "{label}" }
    }
}

#[component]
pub fn ActionButton(label: &'static str, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button { class: BUTTON_THEME, onclick: move |evt| onclick.call(evt), "{label}" }
    }
}

#[component]
pub fn GoBackButton(label: Option<&'static str>) -> Element {
    let label = label.unwrap_or("Go Back");
    rsx! {
        ActionButton {
            label,
            onclick: move |_| {
                navigator().go_back();
            }
        }
    }
}

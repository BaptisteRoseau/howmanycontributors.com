use dioxus::prelude::*;

use crate::assets::{DownArrow, MoonIcon, PcIcon, SunIcon};
use crate::hooks::{Theme, ThemeHandler};

// Inspired by:
// https://github.com/tailwindlabs/tailwindcss.com/blob/a1f4dd1736825483f576922efd472759a5dbe428/src/components/ThemeToggle.js#L71

#[component]
pub fn ThemeSwitcher(theme: Signal<ThemeHandler>) -> Element {
    let onchange = move |event: Event<FormData>| {
        theme.write().set(event.value().into());
    };

    let option_class = move |option_theme: Theme| {
        if theme.read().get() == option_theme {
            "fill-sky-400/20 stroke-sky-500 hover:stroke-sky-700 focus:stroke-sky-700"
        } else {
            "stroke-slate-400 dark:stroke-slate-500  hover:stroke-sky-700 focus:stroke-sky-700"
        }
    };

    let selected = use_signal(|| false);

    rsx! {

        div { class: "flex items-center justify-between",
            div { class: "relative flex items-center ring-1 ring-slate-500/10 rounded-lg shadow-sm p-2 text-slate-600 font-semibold dark:bg-slate-600 dark:ring-0 dark:highlight-white/5 dark:text-slate-200",
                div { class: "w-6 h-6 mr-2 dark:hidden", SunIcon {selected} }
                div { class: "w-6 h-6 mr-2 hidden dark:block", MoonIcon {selected} }
                select {
                    id: "theme",
                    value: theme.read().get().as_str(),
                    onchange: onchange,
                    class: "absolute appearance-none inset-0 w-full h-full opacity-0",
                    option { class: option_class(Theme::Light), value: "light",
                    div {
                        "Light"
                    }
                    }
                    option { class: option_class(Theme::Dark), value: "dark",
                        "Dark"
                    }
                    option { class: option_class(Theme::System), value: "system",
                        "System"
                    }
                }
            }
        }
    }
}

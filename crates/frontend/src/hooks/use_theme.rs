use super::UsePersistent;
use dioxus::prelude::*;
use web_sys::window;

// Do not change this without automatically migrating the password of all clients.
const THEME_KEY: &str = "hmc.theme";
const LIGHT: &str = "light";
const DARK: &str = "dark";

/// State handle for the [`use_theme`] hook.
/// Stored in the users's device.
#[derive(Clone, Debug)]
pub struct ThemeHandler {
    inner: UsePersistent<Option<String>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Theme {
    Light,
    Dark,
    System,
}

impl ThemeHandler {
    /// Save the current theme in the browser.
    pub fn set(&mut self, theme: Theme) {
        match theme {
            Theme::Light => {
                self.inner.set(Some(LIGHT.to_string()));
            }
            Theme::Dark => {
                self.inner.set(Some(DARK.to_string()));
            }
            Theme::System => self.inner.remove(),
        }
        self.set_window_theme()
    }

    /// Get the current theme from the browser.
    pub fn get(&self) -> Theme {
        match self.inner.get() {
            Some(theme) if theme == LIGHT => Theme::Light,
            Some(theme) if theme == DARK => Theme::Dark,
            None | Some(_) => Theme::System,
        }
    }

    /// Get the current theme from the browser.
    ///
    /// Returns either [`Theme::Light`] or [`Theme::Dark`], gets the theme from
    /// the browser in case it is [`Theme::System`].
    pub fn current(&self) -> Theme {
        let theme = self.get();
        match theme {
            Theme::Light | Theme::Dark => theme,
            Theme::System => Self::get_theme_from_system_with_default(Theme::Light),
        }
    }

    /// Adds or remove "dark" in the HTML class name based on the current theme.
    pub(super) fn set_window_theme(&self) {
        let current = self.current();
        let html_classes = window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
            .map(|document_element| document_element.class_list());

        if html_classes.is_some() {
            let html_classes = html_classes.unwrap();
            if current == Theme::Dark {
                let _ = html_classes.add_1(DARK);
            } else {
                let _ = html_classes.remove_1(DARK);
            };
        }
    }

    /// Retrieves the theme from the system.
    ///
    /// If the theme is not retrievable from the system, defaults to `default`.
    fn get_theme_from_system_with_default(default: Theme) -> Theme {
        window()
            .and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok())
            .and_then(|media_queries| media_queries)
            .map_or(default.clone(), |media_queries| {
                if media_queries.matches() {
                    Theme::Dark
                } else {
                    default
                }
            })
    }
}

impl Theme {
    /// lowercase string representation of the [`Theme`] struct variants.
    pub fn as_str(&self) -> &str {
        match self {
            Theme::Light => LIGHT,
            Theme::Dark => DARK,
            Theme::System => "system",
        }
    }

    /// Human-readable string representation of the [`Theme`] struct variants.
    pub fn as_str_pretty(&self) -> &str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::System => "System",
        }
    }
}

impl From<String> for Theme {
    fn from(value: String) -> Self {
        match value.to_string().as_str() {
            LIGHT => Theme::Light,
            DARK => Theme::Dark,
            _ => Theme::System,
        }
    }
}

/// Store the theme in local storage.
pub fn use_theme() -> Signal<ThemeHandler> {
    use_signal(|| {
        let inner: UsePersistent<Option<String>> = UsePersistent::new(THEME_KEY, || None);
        ThemeHandler { inner }
    })
}

pub fn init_theme(){
    use_theme().read().set_window_theme();
}

#[cfg(test)]
mod tests_theme {
    use super::*;

    #[test]
    fn as_str() {
        assert_eq!(Theme::Dark.as_str(), DARK);
        assert_eq!(Theme::Light.as_str(), LIGHT);
        assert_eq!(Theme::System.as_str(), "system");
    }

    #[test]
    fn as_str_pretty() {
        assert_eq!(Theme::Dark.as_str_pretty(), "Dark");
        assert_eq!(Theme::Light.as_str_pretty(), "Light");
        assert_eq!(Theme::System.as_str_pretty(), "System");
    }

    #[test]
    fn from_string() {
        assert_eq!(Theme::Light, String::from(LIGHT).into());
        assert_eq!(Theme::Dark, String::from(DARK).into());
        assert_eq!(Theme::System, String::from("").into());
        assert_eq!(Theme::System, String::from("foo").into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gloo::storage::{LocalStorage, Storage};
    use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
    const THEME_KEY: &str = "hmc-test.theme";

    wasm_bindgen_test_configure!(run_in_browser);

    fn clean() {
        LocalStorage::clear();
    }

    fn theme_handler() -> ThemeHandler {
        ThemeHandler {
            inner: UsePersistent::new(THEME_KEY, || None),
        }
    }

    fn html_element_class_contains_dark() -> bool {
        window()
            .and_then(|window| window.document())
            .and_then(|document| document.document_element())
            .map(|document_element| document_element.class_list())
            .map(|class_list| class_list.contains(DARK))
            .unwrap()
    }

    #[test]
    fn set_and_get() {
        let mut theme_handler = theme_handler();
        for theme in [Theme::Light, Theme::Dark, Theme::System] {
            theme_handler.set(theme.clone());
            assert_eq!(theme, theme_handler.get());
        }
        clean();
    }

    #[test]
    fn current_is_either_light_or_dark() {
        let mut theme_handler = theme_handler();

        theme_handler.set(Theme::Light);
        assert!(matches!(theme_handler.current(), Theme::Light));

        theme_handler.set(Theme::Dark);
        assert!(matches!(theme_handler.current(), Theme::Dark));

        theme_handler.set(Theme::System);
        assert!(matches!(
            theme_handler.current(),
            Theme::Light | Theme::Dark
        ));

        clean();
    }

    #[test]
    fn html_element_modified() {
        let mut theme_handler = theme_handler();

        theme_handler.set(Theme::Light);
        assert!(!html_element_class_contains_dark());

        theme_handler.set(Theme::Dark);
        assert!(html_element_class_contains_dark());

        clean();
    }
}

use crate::pages::Home;

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Routes {
    #[route("/")]
    Home {},
}

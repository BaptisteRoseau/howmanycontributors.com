use crate::pages::{Home, About};

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Routes {
    #[route("/")]
    Home {},
    #[route("/about")]
    About {},
}

use crate::pages::{Home, About, Leaderboard};

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Routes {
    #[route("/")]
    Home {},
    #[route("/leaderboard")]
    Leaderboard {},
    #[route("/about")]
    About {},
}

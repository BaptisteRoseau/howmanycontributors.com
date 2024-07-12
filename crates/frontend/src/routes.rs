use crate::pages::{About, Home, Leaderboard};

use dioxus::prelude::*;

#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Routes {
    #[route("/")]
    #[redirect("/:.._segments", |_segments: Vec<String>| Routes::Home {})]
    Home {},
    #[route("/leaderboard")]
    Leaderboard {},
    #[route("/about")]
    About {},
}

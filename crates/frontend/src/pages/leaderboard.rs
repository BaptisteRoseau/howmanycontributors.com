use crate::components::{Footer, Header, Leaderboard as LeaderboardComponent};

use dioxus::prelude::*;

#[component]
pub fn Leaderboard() -> Element {
    rsx! {
        Header {}
        LeaderboardComponent {}
        Footer {}
    }
}

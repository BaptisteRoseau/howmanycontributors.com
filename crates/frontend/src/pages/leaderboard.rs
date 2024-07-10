use crate::components::{Footer, Header, Leaderboard as LeaderboardComponent};

use dioxus::prelude::*;

#[component]
pub fn Leaderboard() -> Element {
    rsx! {
        body { class: "flex flex-col min-h-screen justify-between",
            body {
                Header {}
                LeaderboardComponent {}
            }
            Footer {}
        }
    }
}

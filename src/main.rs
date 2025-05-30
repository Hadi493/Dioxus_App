use dioxus::prelude::*;
mod Home;
use crate::Home::Home as OneHome;

const MAIN_CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        OneHome {}
    }
}




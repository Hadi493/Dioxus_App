use dioxus::prelude::*;
mod Home;
use crate::Home::Home as HomePage;
mod About;
use crate::About::About as AboutPage;

const MAIN_CSS: Asset = asset!("/assets/main.css");
// const ABOUT_CSS: Asset = asset!("/assets/about.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        HomePage {}
        // document::Link { rel: "stylesheet", href: ABOUT_CSS }
        AboutPage {}
    }
}




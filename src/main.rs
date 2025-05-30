use dioxus::prelude::*;
mod Home;
use Home::Home as HomePage;
mod About;
use crate::About::About as AboutPage;
mod Footer;
use Footer::Footer as FooterPage;

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
        FooterPage {}
    }
}




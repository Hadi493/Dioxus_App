use dioxus::prelude::*;
const PROFILE_IMG: Asset = asset!("/assets/hadialam.png");
const MAIN_CSS: Asset = asset!("/assets/main.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Home {}
    }
}


#[component]
pub fn Home() -> Element {
    rsx! {
        section {
            class: "home-container",
            
            div {
                class: "content",
                
                h1 {
                    class: "name",
                    "Hadi Alam"
                }
                p {
                    class: "tagline",
                    "⚙️ Full Stack Developer | Django | Rust | React | OS Hacker"
                }

                a {
                    href: "https://github.com/Hadi493",
                    class: "btn",
                    target: "_blank",
                    "🔗 GitHub
                }
            }

            img {
                src: PROFILE_IMG,
                class: "profile-image",
                alt: "Hadi Alam"
            }
        }
    }
}


use dioxus::prelude::*;

const PROFILE_IMG: Asset = asset!("/assets/hadialam.png");


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

                h1 {
                    class: "headline",
                    "Full Stack Developer"
                }
                p {
                    class: "tagline",
                    "⚙️ Django | Rust | React | OS Hacker"
                }

                a {
                    href: "https://github.com/Hadi493",
                    class: "btn",
                    target: "_blank",
                    "🔗 GitHub"
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


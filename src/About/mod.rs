use dioxus::prelude::*;

const PROFILE_IMG: Asset = asset!("/assets/hadialam.png");


#[component]
pub fn About() -> Element {
    rsx! {
        section {
            class: "about-container",
            div {
                class: "about-content",
                h2 {
                    class: "about-title",
                    "About Me"
                }
                p {
                    class: "about-description",
                    "I'm a passionate full-stack developer exploring the beauty of Rust, Django, React, and OS hacking. Always curious, always learning."
                }
                a {
                    href: "mailto:hadi@example.com",
                    class: "about-btn",
                    "Get In Touch"
                }
            }
            img {
                src: PROFILE_IMG,
                alt: "Hadi Alam",
                class: "about-image"
            }
        }
    }
}


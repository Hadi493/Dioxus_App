use dioxus::prelude::*;

const PROFILE_IMG: Asset = asset!("/assets/hadialam.png");

pub fn Footer() -> Element {
    rsx! {
        footer {
            class: "footer",
            div {
                class: "footer-content",
                img {
                    src: PROFILE_IMG,
                    alt: "Profile Image",
                    class: "footer-profile-img"
                }
                div {
                    class: "footer-text",
                    "Made with ❤️ by Hadialam"
                }
            }
        }
    }
}

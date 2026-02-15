use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Landing() -> Element {
    rsx! {
        section {
            class: "hero-panel",
            div {
                class: "hero-copy",
                p { class: "eyebrow", "Procedural Song Playback" }
                h2 { "A new medium for recorded music." }
                p {
                    "Proteus explores non-fixed audio playback by combining alternate takes of each track into a unique performance every time you press play."
                }
                div {
                    class: "hero-actions",
                    Link { to: Route::Downloads {}, class: "btn btn-primary", "Get Downloads" }
                    Link { to: Route::About {}, class: "btn btn-ghost", "Read the Story" }
                }
            }

            div {
                class: "hero-art",
                img {
                    src: asset!("/assets/images/icon.png"),
                    alt: "Proteus wave icon",
                }
                p { "Inspired by the Proteus Author interface" }
            }
        }

        section {
            class: "card-grid",
            article {
                class: "info-card",
                h3 { "Variable Playback" }
                p { "One .prot project can produce large numbers of valid combinations from real performances instead of static renders." }
            }
            article {
                class: "info-card",
                h3 { "Artist-Controlled Structure" }
                p { "Creators define tracks, parts, and behavior to keep each playback coherent while still being dynamic." }
            }
            article {
                class: "info-card",
                h3 { "Rust + Desktop Tooling" }
                p { "The authoring workflow is built with modern Rust-driven tooling for performance and maintainability." }
            }
        }
    }
}

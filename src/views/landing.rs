use crate::components::{CtaLink, InfoCard};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Landing() -> Element {
    rsx! {
        section {
            class: "surface-card grid gap-4 bg-gradient-to-br from-[rgba(255,255,255,0.95)] to-[rgba(235,246,252,0.9)] p-5 md:grid-cols-[1.3fr_0.7fr] md:p-6",
            div {
                class: "space-y-4",
                p {
                    class: "font-['Silkscreen'] text-[11px] tracking-[0.06em] text-[var(--primary-deep)]",
                    "Procedural Song Playback"
                }
                h2 {
                    class: "text-3xl font-extrabold leading-tight text-[var(--text)] md:text-5xl",
                    "A new medium for recorded music."
                }
                p {
                    class: "max-w-3xl text-sm leading-7 text-muted md:text-base",
                    "Proteus explores non-fixed audio playback by combining alternate takes of each track into a unique performance every time you press play."
                }
                div { class: "flex flex-wrap gap-2.5",
                    CtaLink {
                        to: Route::Downloads {},
                        label: "Get Downloads".to_string(),
                        primary: true,
                    }
                    CtaLink {
                        to: Route::About {},
                        label: "Read the Story".to_string(),
                        primary: false,
                    }
                }
            }

            div {
                class: "surface-card flex flex-col items-center justify-center bg-gradient-to-b from-[#f8fbfd] to-[#e5f0f8] p-4 text-center",
                img {
                    class: "pixelated-icon w-[72%] max-w-[220px]",
                    src: asset!("/assets/images/icon.png"),
                    alt: "Proteus wave icon",
                }
                p { class: "mt-3 text-sm text-muted", "Inspired by the Proteus Author interface" }
            }
        }

        section {
            class: "grid gap-4 md:grid-cols-3",
            InfoCard {
                title: "Variable Playback".to_string(),
                description: "One .prot project can produce large numbers of valid combinations from real performances instead of static renders.".to_string(),
            }
            InfoCard {
                title: "Artist-Controlled Structure".to_string(),
                description: "Creators define tracks, parts, and behavior to keep each playback coherent while still being dynamic.".to_string(),
            }
            InfoCard {
                title: "Rust + Desktop Tooling".to_string(),
                description: "The authoring workflow is built with modern Rust-driven tooling for performance and maintainability.".to_string(),
            }
        }
    }
}

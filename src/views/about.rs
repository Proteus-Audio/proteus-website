use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        section {
            class: "content-panel",
            h2 { "About Proteus" }
            p {
                "Proteus is an audio format and tooling ecosystem focused on performance-style playback for recorded songs."
            }
            p {
                "The core idea is to capture multiple takes per part and let playback choose combinations in real time, making each listen distinct while preserving a defined structure."
            }
            blockquote {
                "\"It’s possible that our grandchildren will look at us and say ‘You mean people used to listen to the same thing over and over again?’\""
            }
            p {
                "The project draws from procedural audio research, game audio systems, and live performance concepts to reframe how recorded music can behave."
            }

            h3 { "Current Repository Scope" }
            ul {
                li { "Proteus Author: desktop authoring app for .prot projects" }
                li { "Proteus Player: focused playback application" }
                li { "Proteus CLI: Rust command-line parsing and tooling" }
            }

            p {
                class: "note",
                "This page is intentionally concise for now. Add deeper technical and historical details as content is finalized."
            }
        }
    }
}

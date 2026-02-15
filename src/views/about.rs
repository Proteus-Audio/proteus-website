use crate::components::SectionPanel;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    rsx! {
        SectionPanel {
            h2 { class: "text-3xl font-bold text-[var(--text)]", "About Proteus" }
            p {
                class: "mt-3 text-sm leading-7 text-muted md:text-base",
                "Proteus is an audio format and tooling ecosystem focused on performance-style playback for recorded songs."
            }
            p {
                class: "mt-3 text-sm leading-7 text-muted md:text-base",
                "The core idea is to capture multiple takes per part and let playback choose combinations in real time, making each listen distinct while preserving a defined structure."
            }
            blockquote {
                class: "mt-4 rounded-r-lg border-l-4 border-[var(--analog)] bg-[#fffbef] px-4 py-3 text-sm text-[#5a4c25]",
                "\"It’s possible that our grandchildren will look at us and say ‘You mean people used to listen to the same thing over and over again?’\""
            }
            p {
                class: "mt-4 text-sm leading-7 text-muted md:text-base",
                "The project draws from procedural audio research, game audio systems, and live performance concepts to reframe how recorded music can behave."
            }

            h3 { class: "mt-5 text-xl font-semibold text-[var(--text)]", "Current Repository Scope" }
            ul { class: "mt-2 list-disc space-y-1 pl-6 text-sm leading-7 text-muted md:text-base",
                li { "Proteus Author: desktop authoring app for .prot projects" }
                li { "Proteus Player: focused playback application" }
                li { "Proteus CLI: Rust command-line parsing and tooling" }
            }

            p {
                class: "mt-4 text-sm text-muted",
                "This page is intentionally concise for now. Add deeper technical and historical details as content is finalized."
            }
        }
    }
}

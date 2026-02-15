use crate::components::{DownloadCard, SectionPanel};
use dioxus::prelude::*;

#[component]
pub fn Downloads() -> Element {
    rsx! {
        SectionPanel {
            h2 { class: "text-3xl font-bold text-[var(--text)]", "Downloads" }
            p {
                class: "mt-3 text-sm leading-7 text-muted md:text-base",
                "Release links will be finalized once packaging for each target is ready."
            }

            div {
                class: "mt-4 grid gap-4 md:grid-cols-3",
                DownloadCard {
                    name: "Proteus Author".to_string(),
                    description: "Desktop authoring environment for creating and exporting .prot projects.".to_string(),
                }
                DownloadCard {
                    name: "Proteus Player".to_string(),
                    description: "Dedicated player for opening and listening to .prot files.".to_string(),
                }
                DownloadCard {
                    name: "Proteus CLI".to_string(),
                    description: "Command-line utilities for parsing, validating, and automating project workflows.".to_string(),
                }
            }

            p {
                class: "mt-4 text-sm text-muted",
                "Need immediate access? Add direct GitHub release URLs or installer links once they are available."
            }
        }
    }
}

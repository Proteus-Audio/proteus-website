use crate::components::{DownloadCard, SectionPanel};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Downloads() -> Element {
    rsx! {
        SectionPanel {
            h2 { class: "text-3xl font-bold text-[var(--text)]", "Downloads" }
            p {
                class: "mt-3 text-sm leading-7 text-muted md:text-base",
                "Choose a project to see platform-specific installers and release information."
            }

            div {
                class: "mt-4 grid gap-4 md:grid-cols-3",
                DownloadCard {
                    name: "Proteus Author".to_string(),
                    description: "Desktop authoring environment for creating and exporting .prot projects.".to_string(),
                    to: Route::DownloadAuthor {},
                    action_label: "View Downloads".to_string(),
                }
                DownloadCard {
                    name: "Proteus Player".to_string(),
                    description: "Dedicated player for opening and listening to .prot files.".to_string(),
                    to: Route::DownloadPlayer {},
                    action_label: "View Downloads".to_string(),
                }
                DownloadCard {
                    name: "Proteus CLI".to_string(),
                    description: "Command-line utilities for parsing, validating, and automating project workflows.".to_string(),
                    to: Route::DownloadCli {},
                    action_label: "View Downloads".to_string(),
                }
            }
        }
    }
}

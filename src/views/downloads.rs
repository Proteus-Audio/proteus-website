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
                "Choose a project for platform-specific installers and release information."
            }

            div {
                class: "mt-4 grid gap-4 md:grid-cols-3",
                DownloadCard {
                    name: "Proteus Author".to_string(),
                    description: "Desktop app for combinging stems and creating distributable .prot files.".to_string(),
                    to: Route::DownloadAuthor {},
                    action_label: "View Downloads".to_string(),
                }
                DownloadCard {
                    name: "Proteus Player".to_string(),
                    description: "Quicktime-inspired app for playing .prot files in individual windows.".to_string(),
                    to: Route::DownloadPlayer {},
                    action_label: "View Downloads".to_string(),
                }
                DownloadCard {
                    name: "Proteus CLI".to_string(),
                    description: "Command-line tool, useful for testing. A lightweight alternative to the Player.".to_string(),
                    to: Route::DownloadCli {},
                    action_label: "View Downloads".to_string(),
                }
            }
        }
    }
}

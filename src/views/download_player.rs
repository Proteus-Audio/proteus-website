use crate::components::{DesktopAppDownloadPanel, SectionPanel};
use dioxus::prelude::*;

#[component]
pub fn DownloadPlayer() -> Element {
    rsx! {
        SectionPanel {
            DesktopAppDownloadPanel {
                app_name: "Proteus Player".to_string(),
                summary: "Play individual .prot files in a focused desktop listening application.".to_string(),
                release_url: "https://github.com/Proteus-Audio/proteus-player/releases/latest".to_string(),
                repo_url: "https://github.com/Proteus-Audio/proteus-player".to_string(),
            }
        }
    }
}

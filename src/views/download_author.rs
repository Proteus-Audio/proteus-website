use crate::components::{DesktopAppDownloadPanel, SectionPanel};
use dioxus::prelude::*;

#[component]
pub fn DownloadAuthor() -> Element {
    rsx! {
        SectionPanel {
            DesktopAppDownloadPanel {
                app_name: "Proteus Author".to_string(),
                summary: "Author and package .prot projects with the desktop editor.".to_string(),
                manifest_url: "https://github.com/Proteus-Audio/proteus-author/releases/latest/download/downloads.json".to_string(),
                github_latest_api_url: "https://api.github.com/repos/Proteus-Audio/proteus-author/releases/latest".to_string(),
                fallback_release_url: "https://github.com/Proteus-Audio/proteus-author/releases/latest".to_string(),
                repo_url: "https://github.com/Proteus-Audio/proteus-author".to_string(),
            }
        }
    }
}

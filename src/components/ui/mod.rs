//! UI components and download-specific client/server helpers.
//!
//! The module is split by concern so rendering code, platform detection,
//! asset selection, and server manifest fetching can evolve independently.

mod desktop_download_panel;
mod manifest;
mod platform;
mod selection;
mod shared;
mod types;

pub use desktop_download_panel::DesktopAppDownloadPanel;
pub use shared::{CtaLink, DownloadCard, InfoCard, SectionPanel};

use serde::{Deserialize, Serialize};

/// Desktop platform families supported by download selection.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum DesktopOs {
    MacOS,
    Windows,
    Linux,
    Unknown,
}

/// CPU architecture options exposed in the UI.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum DesktopArch {
    X64,
    Arm64,
    Unknown,
}

/// Release asset row normalized for frontend selection.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DownloadAsset {
    pub(crate) name: String,
    pub(crate) url: String,
}

/// Normalized manifest consumed by download panels.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DownloadManifest {
    pub(crate) version: String,
    pub(crate) release_url: String,
    pub(crate) assets: Vec<DownloadAsset>,
}

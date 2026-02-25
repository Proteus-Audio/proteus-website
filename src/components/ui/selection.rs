use super::types::{DesktopArch, DesktopOs, DownloadAsset, DownloadManifest};

fn asset_arch(name: &str) -> DesktopArch {
    let lower = name.to_lowercase();
    if lower.contains("aarch64") || lower.contains("arm64") {
        DesktopArch::Arm64
    } else if lower.contains("x86_64") || lower.contains("amd64") || lower.contains("x64") {
        DesktopArch::X64
    } else {
        DesktopArch::Unknown
    }
}

fn asset_matches_os(name: &str, os: DesktopOs) -> bool {
    let lower = name.to_lowercase();

    match os {
        DesktopOs::MacOS => lower.ends_with(".dmg") || lower.ends_with(".app.tar.gz"),
        DesktopOs::Windows => lower.ends_with(".msi") || lower.ends_with("-setup.exe"),
        DesktopOs::Linux => {
            lower.ends_with(".appimage") || lower.ends_with(".deb") || lower.ends_with(".rpm")
        }
        DesktopOs::Unknown => true,
    }
}

// Scoring allows us to pick the best installer for a given OS/architecture while tolerating
// variant package types in release assets.
fn score_asset(name: &str, os: DesktopOs, arch: DesktopArch) -> i32 {
    let lower = name.to_lowercase();
    let mut score = 0;

    score += match os {
        DesktopOs::MacOS => {
            if lower.ends_with(".dmg") {
                100
            } else if lower.ends_with(".app.tar.gz") {
                80
            } else {
                0
            }
        }
        DesktopOs::Windows => {
            if lower.ends_with(".msi") {
                100
            } else if lower.ends_with("-setup.exe") {
                85
            } else {
                0
            }
        }
        DesktopOs::Linux => {
            if lower.ends_with(".deb") {
                100
            } else if lower.ends_with(".appimage") {
                90
            } else if lower.ends_with(".rpm") {
                80
            } else {
                0
            }
        }
        DesktopOs::Unknown => 50,
    };

    let found_arch = asset_arch(name);
    if arch != DesktopArch::Unknown {
        if found_arch == arch {
            score += 30;
        } else if found_arch != DesktopArch::Unknown {
            score -= 40;
        }
    }

    score
}

pub(crate) fn pick_best_asset(
    manifest: &DownloadManifest,
    os: DesktopOs,
    arch: DesktopArch,
) -> Option<DownloadAsset> {
    manifest
        .assets
        .iter()
        .filter(|asset| asset_matches_os(&asset.name, os))
        .max_by_key(|asset| score_asset(&asset.name, os, arch))
        .cloned()
}

use dioxus::prelude::*;

use super::types::{DesktopArch, DesktopOs};

fn detect_os_from_signals(signals: &[&str]) -> DesktopOs {
    let merged = signals.join(" ").to_lowercase();

    if merged.contains("windows") || merged.contains("win32") || merged.contains("win64") {
        DesktopOs::Windows
    } else if merged.contains("mac os")
        || merged.contains("macintosh")
        || merged.contains("darwin")
        || merged.contains("macintel")
    {
        DesktopOs::MacOS
    } else if merged.contains("linux")
        || merged.contains("x11")
        || merged.contains("ubuntu")
        || merged.contains("fedora")
        || merged.contains("debian")
    {
        DesktopOs::Linux
    } else {
        DesktopOs::Unknown
    }
}

fn detect_arch_from_signals(signals: &[&str]) -> DesktopArch {
    let merged = signals.join(" ").to_lowercase();

    if merged.contains("arm64") || merged.contains("aarch64") || merged.contains("arm") {
        DesktopArch::Arm64
    } else if merged.contains("x86_64")
        || merged.contains("amd64")
        || merged.contains("x64")
        || merged.contains("win64")
        || merged.contains("x86")
    {
        DesktopArch::X64
    } else {
        DesktopArch::Unknown
    }
}

pub(crate) async fn detect_platform_in_browser() -> (DesktopOs, DesktopArch) {
    let script = r#"
        const nav = window?.navigator ?? {};
        const uaDataPlatform = nav.userAgentData?.platform ?? "";
        const uaDataArch = nav.userAgentData?.architecture ?? "";
        const platform = nav.platform ?? "";
        const userAgent = nav.userAgent ?? "";
        const appVersion = nav.appVersion ?? "";
        return `${uaDataPlatform}|||${uaDataArch}|||${platform}|||${userAgent}|||${appVersion}`;
    "#;

    match document::eval(script).join::<String>().await {
        Ok(raw) => {
            let parts = raw.split("|||").collect::<Vec<_>>();
            if parts.len() == 5 {
                let os = detect_os_from_signals(&[parts[0], parts[2], parts[3], parts[4]]);
                let arch = detect_arch_from_signals(&[parts[1], parts[2], parts[3], parts[4]]);
                (os, arch)
            } else {
                let os = detect_os_from_signals(&[&raw]);
                let arch = detect_arch_from_signals(&[&raw]);
                (os, arch)
            }
        }
        Err(_) => (DesktopOs::Unknown, DesktopArch::Unknown),
    }
}

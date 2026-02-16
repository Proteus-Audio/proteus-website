use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DesktopOs {
    MacOS,
    Windows,
    Linux,
    Unknown,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DesktopArch {
    X64,
    Arm64,
    Unknown,
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct DownloadAsset {
    name: String,
    url: String,
}

#[derive(Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct DownloadManifest {
    version: String,
    release_url: String,
    assets: Vec<DownloadAsset>,
}

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

async fn detect_platform_in_browser() -> (DesktopOs, DesktopArch) {
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

async fn fetch_download_manifest(
    manifest_url: &str,
    github_latest_api_url: &str,
) -> Result<DownloadManifest, String> {
    fetch_download_manifest_server(manifest_url.to_string(), github_latest_api_url.to_string())
        .await
        .map_err(|err| format!("Failed to fetch manifest: {err}"))
}

#[post("/api/download-manifest")]
async fn fetch_download_manifest_server(
    manifest_url: String,
    github_latest_api_url: String,
) -> Result<DownloadManifest> {
    #[derive(serde::Deserialize)]
    struct ManifestAssetWire {
        name: Option<String>,
        download_url: Option<String>,
    }

    #[derive(serde::Deserialize)]
    struct ManifestWire {
        version: Option<String>,
        release_url: Option<String>,
        assets: Option<Vec<ManifestAssetWire>>,
    }

    #[derive(serde::Deserialize)]
    struct GithubAssetWire {
        name: Option<String>,
        browser_download_url: Option<String>,
    }

    #[derive(serde::Deserialize)]
    struct GithubReleaseWire {
        tag_name: Option<String>,
        html_url: Option<String>,
        assets: Option<Vec<GithubAssetWire>>,
    }

    fn clean_assets(assets: Vec<(Option<String>, Option<String>)>) -> Vec<DownloadAsset> {
        assets
            .into_iter()
            .filter_map(|(name, url)| match (name, url) {
                (Some(name), Some(url)) if !name.is_empty() && !url.is_empty() => {
                    Some(DownloadAsset { name, url })
                }
                _ => None,
            })
            .collect()
    }

    fn version_from_tag(tag: &str) -> String {
        if let Some((_, rhs)) = tag.rsplit_once("-v") {
            rhs.to_string()
        } else {
            tag.to_string()
        }
    }

    #[cfg(feature = "server")]
    {
        let client = reqwest::Client::builder()
            .user_agent("proteus-website/0.1")
            .build()
            .map_err(|err| ServerFnError::new(format!("failed to build http client: {err}")))?;

        let manifest_attempt = client.get(&manifest_url).send().await;
        if let Ok(response) = manifest_attempt {
            if response.status().is_success() {
                if let Ok(raw) = response.json::<ManifestWire>().await {
                    let assets = clean_assets(
                        raw.assets
                            .unwrap_or_default()
                            .into_iter()
                            .map(|asset| (asset.name, asset.download_url))
                            .collect(),
                    );

                    return Ok(DownloadManifest {
                        version: raw.version.unwrap_or_default(),
                        release_url: raw.release_url.unwrap_or_default(),
                        assets,
                    });
                }
            }
        }

        let response = client
            .get(&github_latest_api_url)
            .header("Accept", "application/vnd.github+json")
            .send()
            .await
            .map_err(|err| ServerFnError::new(format!("github api request failed: {err}")))?;

        if !response.status().is_success() {
            return Err(ServerFnError::new(format!(
                "github api returned status {}",
                response.status()
            ))
            .into());
        }

        let raw = response.json::<GithubReleaseWire>().await.map_err(|err| {
            ServerFnError::new(format!("failed to parse github api response: {err}"))
        })?;

        let assets = clean_assets(
            raw.assets
                .unwrap_or_default()
                .into_iter()
                .map(|asset| (asset.name, asset.browser_download_url))
                .collect(),
        );

        return Ok(DownloadManifest {
            version: raw
                .tag_name
                .as_deref()
                .map(version_from_tag)
                .unwrap_or_default(),
            release_url: raw.html_url.unwrap_or_default(),
            assets,
        });
    }

    #[cfg(not(feature = "server"))]
    {
        let _ = manifest_url;
        let _ = github_latest_api_url;
        Err(ServerFnError::new(
            "server feature is not enabled; cannot fetch manifests from backend",
        )
        .into())
    }
}

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
            if lower.ends_with(".appimage") {
                100
            } else if lower.ends_with(".deb") {
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

fn pick_best_asset(
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

fn os_button_class(active: bool) -> &'static str {
    if active {
        "rounded-sm border border-[var(--primary)] bg-[rgba(25,120,164,0.12)] px-3 py-1.5 text-xs font-semibold text-[var(--primary-deep)]"
    } else {
        "rounded-sm border border-[var(--line)] bg-white px-3 py-1.5 text-xs font-semibold text-[var(--text)] hover:border-[var(--primary)]"
    }
}

fn arch_download_label(os: DesktopOs, arch: DesktopArch) -> &'static str {
    match (os, arch) {
        (DesktopOs::MacOS, DesktopArch::X64) => "Download for Intel Mac",
        (DesktopOs::MacOS, DesktopArch::Arm64) => "Download for Apple Silicon Mac",
        (DesktopOs::Windows, DesktopArch::X64) | (DesktopOs::Linux, DesktopArch::X64) => {
            "Download for x86"
        }
        (DesktopOs::Windows, DesktopArch::Arm64) | (DesktopOs::Linux, DesktopArch::Arm64) => {
            "Download for ARM"
        }
        (_, DesktopArch::X64) => "Download for x86",
        (_, DesktopArch::Arm64) => "Download for ARM",
        _ => "Download",
    }
}

fn arch_choice_label(os: DesktopOs, arch: DesktopArch) -> &'static str {
    match (os, arch) {
        (DesktopOs::MacOS, DesktopArch::X64) => "Intel Mac",
        (DesktopOs::MacOS, DesktopArch::Arm64) => "Apple Silicon",
        (_, DesktopArch::X64) => "x86",
        (_, DesktopArch::Arm64) => "ARM",
        _ => "Unknown",
    }
}

#[component]
pub fn SectionPanel(children: Element) -> Element {
    rsx! {
        section {
            class: "surface-card p-5 md:p-6",
            {children}
        }
    }
}

#[component]
pub fn InfoCard(title: String, description: String) -> Element {
    rsx! {
        article {
            class: "surface-card p-4",
            h3 { class: "text-lg font-semibold text-[var(--text)]", "{title}" }
            p { class: "mt-2 text-sm leading-6 text-muted", "{description}" }
        }
    }
}

#[component]
pub fn DownloadCard(name: String, description: String, to: Route, action_label: String) -> Element {
    rsx! {
        article {
            class: "surface-card p-4",
            h3 { class: "text-lg font-semibold text-[var(--text)]", "{name}" }
            p { class: "mt-2 text-sm leading-6 text-muted", "{description}" }
            Link {
                to,
                class: "mt-4 inline-block rounded-sm border border-[var(--line)] bg-white px-4 py-2 text-sm font-semibold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                "{action_label}"
            }
        }
    }
}

#[component]
pub fn CtaLink(to: Route, label: String, primary: bool) -> Element {
    let class = if primary {
        "rounded-sm bg-[var(--primary)] px-4 py-2.5 text-sm font-bold text-white transition hover:bg-[var(--primary-deep)]"
    } else {
        "rounded-sm border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]"
    };

    rsx! {
        Link {
            to,
            class,
            "{label}"
        }
    }
}

#[component]
pub fn DesktopAppDownloadPanel(
    app_name: String,
    summary: String,
    manifest_url: String,
    github_latest_api_url: String,
    fallback_release_url: String,
    repo_url: String,
) -> Element {
    let mut selected_os = use_signal(|| DesktopOs::Unknown);
    let mut selected_arch = use_signal(|| DesktopArch::Unknown);
    let mut manifest = use_signal(|| None::<DownloadManifest>);
    let mut manifest_error = use_signal(|| None::<String>);
    let mut manifest_loading = use_signal(|| true);

    {
        let manifest_url = manifest_url.clone();
        let github_latest_api_url = github_latest_api_url.clone();
        use_effect(move || {
            let manifest_url = manifest_url.clone();
            let github_latest_api_url = github_latest_api_url.clone();
            spawn(async move {
                let (os, arch) = detect_platform_in_browser().await;
                selected_os.set(os);
                selected_arch.set(match arch {
                    DesktopArch::Unknown => DesktopArch::X64,
                    _ => arch,
                });

                match fetch_download_manifest(&manifest_url, &github_latest_api_url).await {
                    Ok(data) => {
                        manifest.set(Some(data));
                        manifest_error.set(None);
                    }
                    Err(err) => {
                        manifest.set(None);
                        manifest_error.set(Some(err));
                    }
                }
                manifest_loading.set(false);
            });
        });
    }

    let current_os = selected_os();
    let current_arch = selected_arch();
    let selected_asset = manifest()
        .as_ref()
        .and_then(|data| pick_best_asset(data, current_os, current_arch));

    let effective_release_url = manifest()
        .as_ref()
        .and_then(|data| {
            if data.release_url.is_empty() {
                None
            } else {
                Some(data.release_url.clone())
            }
        })
        .unwrap_or_else(|| fallback_release_url.clone());

    let download_href = selected_asset
        .as_ref()
        .map(|asset| asset.url.clone())
        .unwrap_or_else(|| effective_release_url.clone());

    let release_label = manifest()
        .as_ref()
        .map(|data| {
            if data.version.is_empty() {
                "Latest release".to_string()
            } else {
                format!("Latest release: v{}", data.version)
            }
        })
        .unwrap_or_else(|| "Latest release".to_string());

    rsx! {
        h2 { class: "text-3xl font-bold text-[var(--text)]", "{app_name} Downloads" }
        p { class: "mt-3 text-sm leading-7 text-muted md:text-base", "{summary}" }

        div {
            class: "mt-5 surface-card p-4",
            if manifest_loading() {
                p { class: "text-sm text-muted", "Loading release manifest..." }
            }

            if let Some(err) = manifest_error() {
                p {
                    class: "mt-2 text-sm text-[#a64c4c]",
                    "Could not load downloads.json. Falling back to release page. ({err})"
                }
            }

            div { class: if manifest_loading() { "mt-3 flex flex-wrap gap-2" } else { "flex flex-wrap gap-2" },
                button {
                    class: os_button_class(current_os == DesktopOs::MacOS),
                    onclick: move |_| selected_os.set(DesktopOs::MacOS),
                    "macOS"
                }
                button {
                    class: os_button_class(current_os == DesktopOs::Windows),
                    onclick: move |_| selected_os.set(DesktopOs::Windows),
                    "Windows"
                }
                button {
                    class: os_button_class(current_os == DesktopOs::Linux),
                    onclick: move |_| selected_os.set(DesktopOs::Linux),
                    "Linux"
                }
            }

            div { class: "mt-4 flex flex-wrap items-center justify-center gap-2",
                button {
                    class: os_button_class(current_arch == DesktopArch::X64),
                    onclick: move |_| selected_arch.set(DesktopArch::X64),
                    "{arch_choice_label(current_os, DesktopArch::X64)}"
                }
                button {
                    class: os_button_class(current_arch == DesktopArch::Arm64),
                    onclick: move |_| selected_arch.set(DesktopArch::Arm64),
                    "{arch_choice_label(current_os, DesktopArch::Arm64)}"
                }
            }

            div { class: "mx-auto mt-3 w-full max-w-xs",
                a {
                    class: "block flex items-center justify-center text-cyan-700 group pointer-events-none",
                    href: "{download_href}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    div { class: "flex flex-col items-center text-center",
                        div {
                            class: "aspect-square border border-cyan-700 bg-gray-200 rounded-sm p-6 transition group-hover:bg-[var(--primary)]/25",
                            class: "pointer-events-auto",
                            img {
                                class: "pixelated-icon h-20 w-20",
                                src: asset!("/assets/images/icon.png"),
                                alt: "{app_name} icon"
                            }
                        }
                        span { class: "mt-3 text-lg font-bold pointer-events-auto", "{arch_download_label(current_os, current_arch)}" }
                    }
                }
            }

            div { class: "mt-3 flex flex-wrap items-center justify-center gap-2.5",
                a {
                    class: "rounded-sm border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                    href: "{effective_release_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    span { class: "inline-flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            class: "h-4 w-4",
                            path { d: "M12 2C6.477 2 2 6.589 2 12.25c0 4.528 2.865 8.37 6.839 9.726.5.096.682-.223.682-.496 0-.244-.009-.892-.014-1.75-2.782.622-3.369-1.377-3.369-1.377-.455-1.179-1.11-1.493-1.11-1.493-.908-.636.069-.623.069-.623 1.004.072 1.532 1.054 1.532 1.054.892 1.56 2.341 1.11 2.91.848.091-.665.35-1.11.636-1.365-2.221-.259-4.555-1.137-4.555-5.063 0-1.119.39-2.034 1.03-2.751-.103-.26-.447-1.303.098-2.717 0 0 .84-.276 2.75 1.05A9.32 9.32 0 0 1 12 6.84c.85.004 1.706.119 2.505.35 1.909-1.326 2.748-1.05 2.748-1.05.546 1.414.202 2.457.1 2.717.64.717 1.028 1.632 1.028 2.751 0 3.936-2.338 4.801-4.566 5.055.359.319.678.948.678 1.91 0 1.379-.012 2.49-.012 2.829 0 .275.18.596.688.495C19.138 20.616 22 16.776 22 12.25 22 6.589 17.523 2 12 2z" }
                        }
                        "{release_label}"
                    }
                }
                a {
                    class: "rounded-sm border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                    href: "{repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    span { class: "inline-flex items-center gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            class: "h-4 w-4",
                            path { d: "M12 2C6.477 2 2 6.589 2 12.25c0 4.528 2.865 8.37 6.839 9.726.5.096.682-.223.682-.496 0-.244-.009-.892-.014-1.75-2.782.622-3.369-1.377-3.369-1.377-.455-1.179-1.11-1.493-1.11-1.493-.908-.636.069-.623.069-.623 1.004.072 1.532 1.054 1.532 1.054.892 1.56 2.341 1.11 2.91.848.091-.665.35-1.11.636-1.365-2.221-.259-4.555-1.137-4.555-5.063 0-1.119.39-2.034 1.03-2.751-.103-.26-.447-1.303.098-2.717 0 0 .84-.276 2.75 1.05A9.32 9.32 0 0 1 12 6.84c.85.004 1.706.119 2.505.35 1.909-1.326 2.748-1.05 2.748-1.05.546 1.414.202 2.457.1 2.717.64.717 1.028 1.632 1.028 2.751 0 3.936-2.338 4.801-4.566 5.055.359.319.678.948.678 1.91 0 1.379-.012 2.49-.012 2.829 0 .275.18.596.688.495C19.138 20.616 22 16.776 22 12.25 22 6.589 17.523 2 12 2z" }
                        }
                        "View Repository"
                    }
                }
            }
        }

        p {
            class: "mt-4 text-sm text-muted",
            "Auto-selection uses downloads.json data. Switch OS above if needed."
        }
    }
}

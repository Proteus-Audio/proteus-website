use crate::Route;
use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DesktopOs {
    MacOS,
    Windows,
    Linux,
    Unknown,
}

impl DesktopOs {
    fn label(self) -> &'static str {
        match self {
            Self::MacOS => "macOS",
            Self::Windows => "Windows",
            Self::Linux => "Linux",
            Self::Unknown => "Unknown",
        }
    }

    fn installer_hint(self) -> &'static str {
        match self {
            Self::MacOS => ".dmg installer",
            Self::Windows => ".msi installer",
            Self::Linux => ".AppImage or distro package",
            Self::Unknown => "release archive",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DesktopArch {
    X64,
    Arm64,
    Unknown,
}

impl DesktopArch {
    fn label(self) -> &'static str {
        match self {
            Self::X64 => "x64",
            Self::Arm64 => "arm64",
            Self::Unknown => "unknown",
        }
    }
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
        "rounded-lg border border-[var(--primary)] bg-[rgba(25,120,164,0.12)] px-3 py-1.5 text-xs font-semibold text-[var(--primary-deep)]"
    } else {
        "rounded-lg border border-[var(--line)] bg-white px-3 py-1.5 text-xs font-semibold text-[var(--text)] hover:border-[var(--primary)]"
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
            class: "surface-card bg-gradient-to-b from-white to-[#f6fafc] p-4",
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
                class: "mt-4 inline-block rounded-xl border border-[var(--line)] bg-white px-4 py-2 text-sm font-semibold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                "{action_label}"
            }
        }
    }
}

#[component]
pub fn CtaLink(to: Route, label: String, primary: bool) -> Element {
    let class = if primary {
        "rounded-xl bg-[var(--primary)] px-4 py-2.5 text-sm font-bold text-white transition hover:bg-[var(--primary-deep)]"
    } else {
        "rounded-xl border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]"
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
                selected_arch.set(arch);

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

    let best_asset = manifest()
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

    let primary_href = best_asset
        .as_ref()
        .map(|asset| asset.url.clone())
        .unwrap_or_else(|| effective_release_url.clone());

    let primary_cta = if let Some(asset) = &best_asset {
        format!("Download {} ({})", app_name, asset.name)
    } else {
        format!(
            "Get {} for {} ({})",
            app_name,
            current_os.label(),
            current_os.installer_hint()
        )
    };

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
            class: "mt-5 surface-card bg-gradient-to-b from-white to-[#f6fafc] p-4",
            p {
                class: "text-sm text-muted",
                "Detected platform: "
                span { class: "font-semibold text-[var(--text)]", "{current_os.label()}" }
                " / "
                span { class: "font-semibold text-[var(--text)]", "{current_arch.label()}" }
            }

            if manifest_loading() {
                p { class: "mt-2 text-sm text-muted", "Loading release manifest..." }
            }

            if let Some(err) = manifest_error() {
                p {
                    class: "mt-2 text-sm text-[#a64c4c]",
                    "Could not load downloads.json. Falling back to release page. ({err})"
                }
            }

            if let Some(asset) = &best_asset {
                p {
                    class: "mt-2 text-sm text-muted",
                    "Recommended asset: "
                    span { class: "font-semibold text-[var(--text)]", "{asset.name}" }
                }
            }

            div { class: "mt-3 flex flex-wrap gap-2",
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

            div { class: "mt-2 flex flex-wrap gap-2",
                button {
                    class: os_button_class(current_arch == DesktopArch::X64),
                    onclick: move |_| selected_arch.set(DesktopArch::X64),
                    "x64"
                }
                button {
                    class: os_button_class(current_arch == DesktopArch::Arm64),
                    onclick: move |_| selected_arch.set(DesktopArch::Arm64),
                    "arm64"
                }
            }

            div { class: "mt-4 flex flex-wrap gap-2.5",
                a {
                    class: "rounded-xl bg-[var(--primary)] px-4 py-2.5 text-sm font-bold text-white transition hover:bg-[var(--primary-deep)]",
                    href: "{primary_href}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{primary_cta}"
                }
                a {
                    class: "rounded-xl border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                    href: "{effective_release_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{release_label}"
                }
                a {
                    class: "rounded-xl border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                    href: "{repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "View Repository"
                }
            }
        }

        p {
            class: "mt-4 text-sm text-muted",
            "Auto-selection uses downloads.json data. If needed, override OS/architecture before downloading."
        }
    }
}

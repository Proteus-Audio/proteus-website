use dioxus::prelude::*;

use super::manifest::fetch_download_manifest;
use super::platform::detect_platform_in_browser;
use super::selection::pick_best_asset;
use super::types::{DesktopArch, DesktopOs, DownloadManifest};

// Aliases keep callsites readable in rendering code.
type DesktopManifest = DownloadManifest;

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
fn GithubMark() -> Element {
    rsx! {
        svg {
            xmlns: "http://www.w3.org/2000/svg",
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "h-4 w-4",
            path { d: "M12 2C6.477 2 2 6.589 2 12.25c0 4.528 2.865 8.37 6.839 9.726.5.096.682-.223.682-.496 0-.244-.009-.892-.014-1.75-2.782.622-3.369-1.377-3.369-1.377-.455-1.179-1.11-1.493-1.11-1.493-.908-.636.069-.623.069-.623 1.004.072 1.532 1.054 1.532 1.054.892 1.56 2.341 1.11 2.91.848.091-.665.35-1.11.636-1.365-2.221-.259-4.555-1.137-4.555-5.063 0-1.119.39-2.034 1.03-2.751-.103-.26-.447-1.303.098-2.717 0 0 .84-.276 2.75 1.05A9.32 9.32 0 0 1 12 6.84c.85.004 1.706.119 2.505.35 1.909-1.326 2.748-1.05 2.748-1.05.546 1.414.202 2.457.1 2.717.64.717 1.028 1.632 1.028 2.751 0 3.936-2.338 4.801-4.566 5.055.359.319.678.948.678 1.91 0 1.379-.012 2.49-.012 2.829 0 .275.18.596.688.495C19.138 20.616 22 16.776 22 12.25 22 6.589 17.523 2 12 2z" }
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
    let mut manifest = use_signal(|| None::<DesktopManifest>);
    let mut manifest_error = use_signal(|| None::<String>);
    let mut manifest_loading = use_signal(|| true);

    {
        let manifest_url = manifest_url.clone();
        let github_latest_api_url = github_latest_api_url.clone();
        use_effect(move || {
            let manifest_url = manifest_url.clone();
            let github_latest_api_url = github_latest_api_url.clone();
            spawn(async move {
                // Detect once on mount, then allow manual OS/arch overrides from UI controls.
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
            class: "mt-5 surface-card p-4 relative",
            if manifest_loading() {
                div {
                    class: "absolute top-2 right-2",
                    p { class: "text-sm text-muted", "Loading release manifest..." }
                }
            }

            if let Some(err) = manifest_error() {
                p {
                    class: "mt-2 text-sm text-[#a64c4c]",
                    "Could not load downloads.json. Falling back to release page. ({err})"
                }
            }

            div {
                class: "flex flex-wrap gap-2",
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
                            class: "aspect-square border border-cyan-700 bg-gray-200 rounded-sm p-6 transition group-hover:bg-[var(--primary)]/25 pointer-events-auto",
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
                        GithubMark {}
                        "{release_label}"
                    }
                }
                a {
                    class: "rounded-sm border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                    href: "{repo_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    span { class: "inline-flex items-center gap-2",
                        GithubMark {}
                        "View Repository"
                    }
                }
            }

            if current_os == DesktopOs::MacOS {
                div {
                    class: "mt-4 rounded-sm border border-amber-200 bg-amber-50 px-4 py-3 text-left",
                    p {
                        class: "text-sm font-semibold text-amber-900",
                        "Apple Gatekeeper note"
                    }
                    p {
                        class: "mt-1 text-sm leading-6 text-amber-900/90",
                        "If macOS blocks the app, right-click the app and choose Open, or go to System Settings > Privacy & Security and click Open Anyway."
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

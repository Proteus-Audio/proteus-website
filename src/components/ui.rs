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

fn detect_os_from_user_agent(ua: &str) -> DesktopOs {
    detect_os_from_signals(&[ua])
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

async fn detect_os_in_browser() -> DesktopOs {
    let script = r#"
        const nav = window?.navigator ?? {};
        const uaDataPlatform = nav.userAgentData?.platform ?? "";
        const platform = nav.platform ?? "";
        const userAgent = nav.userAgent ?? "";
        const appVersion = nav.appVersion ?? "";
        return `${uaDataPlatform}|||${platform}|||${userAgent}|||${appVersion}`;
    "#;

    match document::eval(script).join::<String>().await {
        Ok(raw) => {
            let parts = raw.split("|||").collect::<Vec<_>>();
            if parts.len() == 4 {
                detect_os_from_signals(&[parts[0], parts[1], parts[2], parts[3]])
            } else {
                detect_os_from_user_agent(&raw)
            }
        }
        Err(_) => DesktopOs::Unknown,
    }
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
    release_url: String,
    repo_url: String,
) -> Element {
    let mut selected_os = use_signal(|| DesktopOs::Unknown);

    use_effect(move || {
        spawn(async move {
            let detected = detect_os_in_browser().await;
            selected_os.set(detected);
        });
    });

    let current_os = selected_os();
    let primary_cta = format!(
        "Get {} for {} ({})",
        app_name,
        current_os.label(),
        current_os.installer_hint()
    );

    rsx! {
        h2 { class: "text-3xl font-bold text-[var(--text)]", "{app_name} Downloads" }
        p { class: "mt-3 text-sm leading-7 text-muted md:text-base", "{summary}" }

        div {
            class: "mt-5 surface-card bg-gradient-to-b from-white to-[#f6fafc] p-4",
            p {
                class: "text-sm text-muted",
                "Detected platform: "
                span { class: "font-semibold text-[var(--text)]", "{current_os.label()}" }
            }
            p {
                class: "mt-1 text-sm text-muted",
                "Recommended package: "
                span { class: "font-semibold text-[var(--text)]", "{current_os.installer_hint()}" }
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

            div { class: "mt-4 flex flex-wrap gap-2.5",
                a {
                    class: "rounded-xl bg-[var(--primary)] px-4 py-2.5 text-sm font-bold text-white transition hover:bg-[var(--primary-deep)]",
                    href: "{release_url}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{primary_cta}"
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
            "If the auto-detected platform is incorrect, choose another OS above before opening releases."
        }
    }
}

use crate::components::SectionPanel;
use dioxus::prelude::*;

#[component]
pub fn DownloadCli() -> Element {
    rsx! {
        SectionPanel {
            h2 { class: "text-3xl font-bold text-[var(--text)]", "Proteus CLI Downloads" }
            p {
                class: "mt-3 text-sm leading-7 text-muted md:text-base",
                "The CLI is distributed through GitHub releases and can be used in automation, validation, and tooling workflows."
            }

            div {
                class: "mt-5 surface-card bg-gradient-to-b from-white to-[#f6fafc] p-4",
                h3 { class: "text-lg font-semibold text-[var(--text)]", "Install Options" }
                ul { class: "mt-2 list-disc space-y-1 pl-6 text-sm leading-7 text-muted",
                    li { "Download a prebuilt binary from the latest release" }
                    li { "Build from source with Cargo for your target environment" }
                    li { "Use the repository for CI/CD scripting and local tooling" }
                }

                div { class: "mt-4 flex flex-wrap gap-2.5",
                    a {
                        class: "rounded-xl bg-[var(--primary)] px-4 py-2.5 text-sm font-bold text-white transition hover:bg-[var(--primary-deep)]",
                        href: "https://github.com/Proteus-Audio/proteus-cli/releases/latest",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "Open Latest Release"
                    }
                    a {
                        class: "rounded-xl border border-[var(--line)] bg-white px-4 py-2.5 text-sm font-bold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                        href: "https://github.com/Proteus-Audio/proteus-cli",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "View Repository"
                    }
                }
            }
        }
    }
}

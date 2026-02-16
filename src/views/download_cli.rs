use crate::components::SectionPanel;
use dioxus::prelude::*;

#[component]
pub fn DownloadCli() -> Element {
    rsx! {
        SectionPanel {
            h2 { class: "text-3xl font-bold text-[var(--text)]", "Proteus CLI Downloads" }
            p {
                class: "mt-3 text-sm leading-7 text-muted md:text-base",
                "The CLI is distributed through Cargo and is primarily useful as a lightweight alternative to the Proteus Player application."
            }

            div {
                class: "mt-5 surface-card p-4",
                h3 { class: "text-lg font-semibold text-[var(--text)]", "Install Options" }
                div { class: "mt-2 list-disc space-y-1 pl-6 text-sm leading-7 text-muted",
                    "Currently the CLI is only available through Cargo but can be installed with the command ",
                    code { class: "bg-gray-100 border border-gray-200 px-1.5 py-0.5 rounded-sm", "cargo install proteus-cli" },
                    "."
                }

                div { class: "mt-4 flex flex-wrap gap-2.5",
                    a {
                        class: "rounded-sm bg-[var(--primary)] px-4 py-2.5 text-sm font-bold text-white transition hover:bg-[var(--primary-deep)]",
                        href: "https://github.com/Proteus-Audio/proteus-core",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        "View Repository"
                    }
                }
            }
        }
    }
}

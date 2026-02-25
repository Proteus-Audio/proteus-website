use crate::components::{DownloadCard, SectionPanel};
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn Downloads() -> Element {
    rsx! {
        div {
            class: "space-y-6",

            SectionPanel {
                h2 { class: "text-3xl font-bold text-[var(--text)]", "Downloads" }
                p {
                    class: "mt-3 text-sm leading-7 text-muted md:text-base",
                    "Choose a project for platform-specific installers and release information."
                }

                div {
                    class: "mt-4 grid gap-4 md:grid-cols-3",
                    DownloadCard {
                        name: "Proteus Author".to_string(),
                        description: "Desktop app for combinging stems and creating distributable .prot files.".to_string(),
                        to: Route::DownloadAuthor {},
                        action_label: "View Downloads".to_string(),
                    }
                    DownloadCard {
                        name: "Proteus Player".to_string(),
                        description: "Quicktime-inspired app for playing .prot files in individual windows.".to_string(),
                        to: Route::DownloadPlayer {},
                        action_label: "View Downloads".to_string(),
                    }
                    DownloadCard {
                        name: "Proteus CLI".to_string(),
                        description: "Command-line tool, useful for testing. A lightweight alternative to the Player.".to_string(),
                        to: Route::DownloadCli {},
                        action_label: "View Downloads".to_string(),
                    }
                }
            }

            SectionPanel {
                h2 { class: "text-2xl font-bold text-[var(--text)]", "File Downloads" }
                p {
                    class: "mt-3 text-sm leading-7 text-muted md:text-base",
                    "Download sample files"
                }

                div { class: "mt-4 grid",
                    article {
                        class: "p-4 grid grid-cols-[1fr_9rem] items-center even:bg-gray-100/50 first:border-t border-b border-gray-200",
                        h3 { class: "md:hidden col-span-2 text-lg font-semibold text-[var(--text)]", "Demo (Dry)" }
                        div {
                            class: "flex items-center gap-4",
                            h3 { class: "hidden md:block text-lg font-semibold text-[var(--text)]", "Demo (Dry)" }
                            p { class: "text-sm leading-6 text-muted", "A short example .prot file with 150,994,944 possible combinations." }
                        }
                        a {
                            class: "inline-block rounded-sm border border-[var(--line)] bg-white px-4 py-2 text-sm font-semibold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                            href: "/examples/demo.prot",
                            download: "demo.prot",
                            "Download File"
                        }
                    }

                    article {
                        class: "p-4 grid grid-cols-[1fr_9rem] items-center even:bg-gray-100/50 first:border-t border-b border-gray-200",
                        h3 { class: "md:hidden col-span-2 text-lg font-semibold text-[var(--text)]", "Demo (Effects)" }
                        div {
                            class: "flex items-center gap-4",
                            h3 { class: "hidden md:block text-lg font-semibold text-[var(--text)]", "Demo (Effects)" }
                            p { class: "text-sm leading-6 text-muted", "The same example .prot file with built in effects applied after file combination." }
                        }
                        a {
                            class: "inline-block rounded-sm border border-[var(--line)] bg-white px-4 py-2 text-sm font-semibold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                            href: "/examples/demo-effects.prot",
                            download: "demo-effects.prot",
                            "Download File"
                        }
                    }

                    article {
                        class: "p-4 gap-2 grid grid-cols-[1fr_9rem] items-center even:bg-gray-100/50 first:border-t border-b border-gray-200",
                        h3 {
                            class: "md:hidden col-span-2 text-lg font-semibold text-[var(--text)]",
                           "Demo (Full Length)"
                        }
                        div {
                            class: "flex items-center gap-4",
                            h3 { class: "hidden md:block text-lg font-semibold text-[var(--text)]", "Demo (Full Length)" }
                            p { class: "mt-2 text-sm leading-6 text-muted", "Very rough demo of a song that I’m currently recording to show file playback on a full-length track." }
                        }
                        a {
                            class: "inline-block rounded-sm border border-[var(--line)] bg-white px-4 py-2 text-sm font-semibold text-[var(--text)] transition hover:border-[var(--primary)] hover:text-[var(--primary-deep)]",
                            href: "/examples/demo-full-length.prot",
                            download: "demo-full-length.prot",
                            "Download File"
                        }
                    }
                }
            }
        }
    }
}

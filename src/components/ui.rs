use crate::Route;
use dioxus::prelude::*;

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
pub fn DownloadCard(name: String, description: String) -> Element {
    rsx! {
        article {
            class: "surface-card p-4",
            h3 { class: "text-lg font-semibold text-[var(--text)]", "{name}" }
            p { class: "mt-2 text-sm leading-6 text-muted", "{description}" }
            button {
                class: "mt-4 rounded-xl border border-[#d8e0e7] bg-[#eef2f6] px-4 py-2 text-sm font-semibold text-[#90a0ad]",
                disabled: true,
                "Coming Soon"
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

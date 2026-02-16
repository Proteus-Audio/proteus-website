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

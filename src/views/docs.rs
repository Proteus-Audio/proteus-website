use crate::components::SectionPanel;
use crate::Route;
use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};

mod registry {
    include!(concat!(env!("OUT_DIR"), "/docs_registry.rs"));
}

#[component]
pub fn DocsIndex() -> Element {
    rsx! {
        DocsView {
            path: String::new(),
        }
    }
}

#[component]
pub fn DocsPage(segments: Vec<String>) -> Element {
    rsx! {
        DocsView {
            path: segments.join("/"),
        }
    }
}

#[component]
fn DocsView(path: String) -> Element {
    let current_path = normalize_path(&path);
    let page = find_page(&current_path);

    rsx! {
        SectionPanel {
            div { class: "grid gap-6 lg:grid-cols-[14rem_1fr]",
                aside { class: "lg:border-r lg:border-[var(--line)] lg:pr-4",
                    div { class: "sticky top-4",
                        h2 { class: "font-silkscreen text-sm tracking-[0.06em] text-[var(--primary-deep)]", "Docs" }
                        nav { class: "mt-3 flex flex-col gap-1",
                            for doc in registry::DOC_PAGES {
                                Link {
                                    to: route_for_doc(doc.path),
                                    class: sidebar_link_class(doc.path == current_path),
                                    style: sidebar_indent(doc.path),
                                    "{doc.title}"
                                }
                            }
                        }
                    }
                }

                article {
                    class: "min-w-0",
                    match page {
                        Some(doc) => rsx! {
                            div {
                                class: "docs-content",
                                dangerous_inner_html: "{markdown_to_html(doc.source)}"
                            }
                        },
                        None => rsx! {
                            div { class: "rounded-sm border border-amber-200 bg-amber-50 p-4 text-amber-900",
                                h1 { class: "text-2xl font-bold", "Page not found" }
                                p { class: "mt-2 text-sm leading-6", "No markdown document exists for /docs/{current_path}." }
                                Link {
                                    to: Route::DocsIndex {},
                                    class: "mt-4 inline-flex rounded-sm border border-amber-300 bg-white px-3 py-2 text-sm font-semibold text-amber-950 hover:bg-amber-100",
                                    "Back to docs"
                                }
                            }
                        },
                    }
                }
            }
        }
    }
}

fn find_page(path: &str) -> Option<&'static registry::DocPage> {
    registry::DOC_PAGES.iter().find(|page| page.path == path)
}

fn markdown_to_html(source: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(source, options);
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

fn normalize_path(path: &str) -> String {
    path.trim_matches('/').to_string()
}

fn route_for_doc(path: &str) -> Route {
    if path.is_empty() {
        Route::DocsIndex {}
    } else {
        Route::DocsPage {
            segments: path.split('/').map(ToOwned::to_owned).collect(),
        }
    }
}

fn sidebar_link_class(is_active: bool) -> &'static str {
    if is_active {
        "rounded-sm border border-[rgba(25,120,164,0.35)] bg-[rgba(25,120,164,0.12)] px-3 py-2 text-sm font-semibold text-[var(--primary-deep)]"
    } else {
        "rounded-sm border border-transparent px-3 py-2 text-sm font-semibold text-[var(--text)] transition hover:bg-[rgba(25,120,164,0.08)] hover:text-[var(--primary-deep)]"
    }
}

fn sidebar_indent(path: &str) -> String {
    let depth = path.matches('/').count();
    format!("margin-left: {}rem;", depth as f32 * 0.75)
}

use crate::components::SectionPanel;
use crate::Route;
use dioxus::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use std::collections::HashSet;

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
    let sidebar = build_sidebar();
    let expanded_paths = use_signal(|| expanded_paths_for(&current_path));

    rsx! {
        SectionPanel {
            div { class: "grid gap-6 lg:grid-cols-[14rem_1fr]",
                aside { class: "lg:border-r lg:border-[var(--line)] lg:pr-4",
                    div { class: "sticky top-4",
                        h2 { class: "font-silkscreen text-sm tracking-[0.06em] text-[var(--primary-deep)]", "Docs" }
                        nav { class: "mt-3 flex flex-col gap-1",
                            for node in sidebar {
                                SidebarNodeView {
                                    node,
                                    current_path: current_path.clone(),
                                    expanded_paths,
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

#[derive(Clone, Debug, PartialEq, Eq)]
struct SidebarNode {
    path: String,
    title: String,
    has_page: bool,
    children: Vec<SidebarNode>,
}

#[component]
fn SidebarNodeView(
    node: SidebarNode,
    current_path: String,
    mut expanded_paths: Signal<HashSet<String>>,
) -> Element {
    let has_children = !node.children.is_empty();
    let is_active = node.has_page && node.path == current_path;
    let is_expanded = has_children
        && (expanded_paths.read().contains(&node.path)
            || path_contains_current(&node.path, &current_path));
    let depth = node.path.matches('/').count();
    let indent = format!("padding-left: {}rem;", depth as f32 * 0.75);
    let toggle_label = if is_expanded { "-" } else { "+" };
    let title = node.title.clone();
    let path = node.path.clone();

    rsx! {
        div {
            div {
                class: "flex items-center gap-1",
                style: "{indent}",

                if has_children {
                    button {
                        r#type: "button",
                        class: "docs-sidebar-toggle",
                        aria_label: if is_expanded { "Collapse {title}" } else { "Expand {title}" },
                        aria_expanded: "{is_expanded}",
                        onclick: move |_| {
                            expanded_paths.with_mut(|paths| {
                                if paths.contains(&path) {
                                    paths.remove(&path);
                                } else {
                                    paths.insert(path.clone());
                                }
                            });
                        },
                        "{toggle_label}"
                    }
                } else {
                    span { class: "docs-sidebar-spacer" }
                }

                if node.has_page {
                    Link {
                        to: route_for_doc(&node.path),
                        class: sidebar_link_class(is_active),
                        "{node.title}"
                    }
                } else {
                    button {
                        r#type: "button",
                        class: sidebar_group_button_class(is_expanded),
                        aria_expanded: "{is_expanded}",
                        onclick: move |_| {
                            expanded_paths.with_mut(|paths| {
                                if paths.contains(&node.path) {
                                    paths.remove(&node.path);
                                } else {
                                    paths.insert(node.path.clone());
                                }
                            });
                        },
                        "{node.title}"
                    }
                }
            }

            if is_expanded {
                div { class: "mt-1 flex flex-col gap-1",
                    for child in node.children {
                        SidebarNodeView {
                            node: child,
                            current_path: current_path.clone(),
                            expanded_paths,
                        }
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
        "flex min-w-0 flex-1 rounded-sm border border-[rgba(25,120,164,0.35)] bg-[rgba(25,120,164,0.12)] px-3 py-2 text-sm font-semibold text-[var(--primary-deep)]"
    } else {
        "flex min-w-0 flex-1 rounded-sm border border-transparent px-3 py-2 text-sm font-semibold text-[var(--text)] transition hover:bg-[rgba(25,120,164,0.08)] hover:text-[var(--primary-deep)]"
    }
}

fn sidebar_group_button_class(is_expanded: bool) -> &'static str {
    if is_expanded {
        "flex min-w-0 flex-1 rounded-sm border border-transparent px-3 py-2 text-left text-sm font-semibold text-[var(--primary-deep)] transition hover:bg-[rgba(25,120,164,0.08)]"
    } else {
        "flex min-w-0 flex-1 rounded-sm border border-transparent px-3 py-2 text-left text-sm font-semibold text-[var(--text)] transition hover:bg-[rgba(25,120,164,0.08)] hover:text-[var(--primary-deep)]"
    }
}

fn build_sidebar() -> Vec<SidebarNode> {
    let mut roots = Vec::new();

    for doc in registry::DOC_PAGES {
        insert_sidebar_page(&mut roots, doc.path, doc.title);
    }

    roots
}

fn insert_sidebar_page(nodes: &mut Vec<SidebarNode>, path: &str, title: &str) {
    if path.is_empty() {
        nodes.insert(
            0,
            SidebarNode {
                path: String::new(),
                title: title.to_string(),
                has_page: true,
                children: Vec::new(),
            },
        );
        return;
    }

    let segments: Vec<&str> = path.split('/').collect();
    insert_sidebar_segments(nodes, &segments, title, String::new());
}

fn insert_sidebar_segments(
    nodes: &mut Vec<SidebarNode>,
    segments: &[&str],
    title: &str,
    parent_path: String,
) {
    let segment = segments[0];
    let node_path = if parent_path.is_empty() {
        segment.to_string()
    } else {
        format!("{parent_path}/{segment}")
    };

    let index = nodes
        .iter()
        .position(|node| node.path == node_path)
        .unwrap_or_else(|| {
            nodes.push(SidebarNode {
                path: node_path.clone(),
                title: humanize_slug(segment),
                has_page: false,
                children: Vec::new(),
            });
            nodes.len() - 1
        });

    if segments.len() == 1 {
        nodes[index].title = title.to_string();
        nodes[index].has_page = true;
    } else {
        insert_sidebar_segments(&mut nodes[index].children, &segments[1..], title, node_path);
    }
}

fn humanize_slug(slug: &str) -> String {
    slug.split(['-', '_'])
        .filter(|word| !word.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().chain(chars).collect::<String>(),
                None => String::new(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn expanded_paths_for(current_path: &str) -> HashSet<String> {
    let mut expanded = HashSet::new();
    let mut segments = current_path.split('/').collect::<Vec<_>>();

    if !current_path.is_empty() {
        expanded.insert(current_path.to_string());
    }

    while segments.len() > 1 {
        segments.pop();
        expanded.insert(segments.join("/"));
    }

    expanded
}

fn path_contains_current(path: &str, current_path: &str) -> bool {
    !path.is_empty()
        && current_path.len() > path.len()
        && current_path.starts_with(path)
        && current_path.as_bytes().get(path.len()) == Some(&b'/')
}

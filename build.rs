use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct DocPage {
    path: String,
    title: String,
    source: String,
}

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=docs");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let docs_dir = manifest_dir.join("docs");
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let registry_path = out_dir.join("docs_registry.rs");

    let mut pages = Vec::new();
    if docs_dir.exists() {
        collect_docs(&docs_dir, &docs_dir, &mut pages)?;
    }
    pages.sort_by(|a, b| sort_key(&a.path).cmp(&sort_key(&b.path)));

    let mut generated = String::from(
        "#[derive(Clone, Copy, Debug, PartialEq, Eq)]\n\
         pub struct DocPage {\n\
         \tpub path: &'static str,\n\
         \tpub title: &'static str,\n\
         \tpub source: &'static str,\n\
         }\n\n\
         pub const DOC_PAGES: &[DocPage] = &[\n",
    );

    for page in pages {
        generated.push_str(&format!(
            "\tDocPage {{ path: {:?}, title: {:?}, source: {:?} }},\n",
            page.path, page.title, page.source
        ));
    }

    generated.push_str("];\n");
    fs::write(registry_path, generated)
}

fn collect_docs(root: &Path, dir: &Path, pages: &mut Vec<DocPage>) -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", dir.display());

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_docs(root, &path, pages)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
            continue;
        }

        println!("cargo:rerun-if-changed={}", path.display());

        let source = fs::read_to_string(&path)?;
        let relative = path
            .strip_prefix(root)
            .expect("doc path should be under docs");
        let route_path = route_path_for(relative);
        let title = title_for(&source, &route_path);

        pages.push(DocPage {
            path: route_path,
            title,
            source,
        });
    }

    Ok(())
}

fn route_path_for(relative: &Path) -> String {
    let mut parts: Vec<String> = relative
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .map(|part| part.trim_end_matches(".md").to_string())
        .collect();

    if parts.last().map(|part| part == "index").unwrap_or(false) {
        parts.pop();
    }

    parts.join("/")
}

fn title_for(source: &str, route_path: &str) -> String {
    source
        .lines()
        .find_map(|line| line.strip_prefix("# ").map(str::trim))
        .filter(|title| !title.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| {
            route_path
                .rsplit('/')
                .find(|segment| !segment.is_empty())
                .map(humanize_slug)
                .unwrap_or_else(|| "Docs".to_string())
        })
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

fn sort_key(path: &str) -> String {
    if path.is_empty() {
        String::new()
    } else {
        format!("{}{}", path.matches('/').count() + 1, path)
    }
}

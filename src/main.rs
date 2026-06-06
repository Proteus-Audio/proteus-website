use dioxus::prelude::*;

use views::{
    About, DocsIndex, DocsPage, DownloadAuthor, DownloadCli, DownloadPlayer, Downloads, Landing,
    SiteLayout,
};

mod components;
mod views;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(SiteLayout)]
        #[route("/")]
        Landing {},
        #[route("/about")]
        About {},
        #[route("/downloads")]
        Downloads {},
        #[route("/downloads/author")]
        DownloadAuthor {},
        #[route("/downloads/player")]
        DownloadPlayer {},
        #[route("/downloads/cli")]
        DownloadCli {},
        #[route("/docs")]
        DocsIndex {},
        #[route("/docs/:..segments")]
        DocsPage { segments: Vec<String> },
}

const SITE_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "180x180", href: "/icons/favicon-180.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "152x152", href: "/icons/favicon-152.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "144x144", href: "/icons/favicon-144.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "120x120", href: "/icons/favicon-120.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "114x114", href: "/icons/favicon-114.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "76x76", href: "/icons/favicon-76.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "72x72", href: "/icons/favicon-72.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "60x60", href: "/icons/favicon-60.png" }
        document::Link { rel: "apple-touch-icon", r#type: "image/png", sizes: "57x57", href: "/icons/favicon-57.png" }

        document::Link { rel: "icon", r#type: "image/png", sizes: "196x196", href: "/icons/favicon-196.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "160x160", href: "/icons/favicon-160.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "95x95", href: "/icons/favicon-95.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "64x64", href: "/icons/favicon-64.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "32x32", href: "/icons/favicon-32.png" }
        document::Link { rel: "icon", r#type: "image/png", sizes: "16x16", href: "/icons/favicon-16.png" }

        document::Link { rel: "icon", sizes: "48x48 32x32 16x16", href: "/icons/favicon.ico" }
        document::Link { rel: "shortcut icon", href: "/icons/favicon.ico" }
        document::Link { rel: "manifest", href: "/icons/manifest.json" }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: SITE_CSS }
        Router::<Route> {}
    }
}

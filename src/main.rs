use dioxus::prelude::*;

use views::{About, DownloadAuthor, DownloadCli, DownloadPlayer, Downloads, Landing, SiteLayout};

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
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const SITE_CSS: Asset = asset!("/assets/styling/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: SITE_CSS }
        Router::<Route> {}
    }
}

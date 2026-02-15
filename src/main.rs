use dioxus::prelude::*;

use views::{About, Downloads, Landing, SiteLayout};

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
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const SITE_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: SITE_CSS }
        Router::<Route> {}
    }
}

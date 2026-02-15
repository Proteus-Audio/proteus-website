use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SiteLayout() -> Element {
    rsx! {
        div {
            class: "site-shell",
            header {
                class: "site-header",
                div {
                    class: "brand-lockup",
                    img {
                        class: "brand-icon",
                        src: asset!("/assets/images/icon.png"),
                        alt: "Proteus Audio wave icon"
                    }
                    div {
                        p { class: "brand-kicker", "Proteus Audio" }
                        h1 { class: "brand-name", "Proteus" }
                    }
                }

                nav {
                    class: "site-nav",
                    Link { to: Route::Landing {}, class: "site-nav-link", "Landing" }
                    Link { to: Route::About {}, class: "site-nav-link", "About" }
                    Link { to: Route::Downloads {}, class: "site-nav-link", "Downloads" }
                }
            }

            main {
                class: "site-main",
                Outlet::<Route> {}
            }

            footer {
                class: "site-footer",
                p { "Proteus Audio Project" }
            }
        }
    }
}

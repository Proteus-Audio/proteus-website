use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SiteLayout() -> Element {
    let current: Route = use_route();
    let (is_landing, is_about, is_downloads) = match &current {
        Route::Landing {} => (true, false, false),
        Route::About {} => (false, true, false),
        Route::Downloads {}
        | Route::DownloadAuthor {}
        | Route::DownloadPlayer {}
        | Route::DownloadCli {} => (false, false, true),
    };

    let nav_inactive = "rounded-sm border border-transparent px-3.5 py-2 text-sm font-semibold text-[var(--text)] transition hover:bg-[rgba(25,120,164,0.08)] hover:text-[var(--primary-deep)]";
    let nav_active = "rounded-sm border border-[rgba(25,120,164,0.35)] bg-[rgba(25,120,164,0.12)] px-3.5 py-2 text-sm font-semibold text-[var(--primary-deep)]";

    rsx! {
        div { class: "min-h-screen px-3 py-4 md:px-6",
            div { class: "mx-auto w-full max-w-6xl",
                header {
                    class: "surface-card flex flex-col gap-4 bg-white p-4 md:flex-row md:items-center md:justify-between",
                    div {
                        class: "flex items-center gap-3",
                        img {
                            class: "h-11 w-11 pixelated-icon",
                            src: asset!("/assets/images/icon.png"),
                            alt: "Proteus Audio wave icon"
                        }
                        div {
                            p {
                                class: "m-0 text-[11px] uppercase tracking-[0.08em] text-muted",
                                "Proteus Audio"
                            }
                            h1 {
                                class: "font-silkscreen m-0 text-lg tracking-[0.04em]",
                                "Proteus"
                            }
                        }
                    }

                    nav { class: "flex flex-wrap gap-1.5",
                        Link { to: Route::Landing {}, class: if is_landing { nav_active } else { nav_inactive }, "Home" }
                        Link { to: Route::About {}, class: if is_about { nav_active } else { nav_inactive }, "About" }
                        Link { to: Route::Downloads {}, class: if is_downloads { nav_active } else { nav_inactive }, "Downloads" }
                    }
                }

                main { class: "mt-4 grid gap-4", Outlet::<Route> {} }

                footer { class: "mt-4 text-center text-sm text-muted", p { "Proteus Audio Project" } }
            }
        }
    }
}

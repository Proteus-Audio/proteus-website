use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn SiteLayout() -> Element {
    let nav_class = "rounded-lg px-3.5 py-2 text-sm font-semibold text-[var(--text)] transition hover:bg-[rgba(25,120,164,0.1)] hover:text-[var(--primary-deep)]";

    rsx! {
        div { class: "min-h-screen px-3 py-4 md:px-6",
            div { class: "mx-auto w-full max-w-6xl",
                header {
                    class: "surface-card flex flex-col gap-4 bg-[rgba(255,255,255,0.82)] p-4 backdrop-blur-sm md:flex-row md:items-center md:justify-between",
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
                                class: "m-0 font-['Silkscreen'] text-lg tracking-[0.04em]",
                                "Proteus"
                            }
                        }
                    }

                    nav { class: "flex flex-wrap gap-1.5",
                        Link { to: Route::Landing {}, class: nav_class, "Landing" }
                        Link { to: Route::About {}, class: nav_class, "About" }
                        Link { to: Route::Downloads {}, class: nav_class, "Downloads" }
                    }
                }

                main { class: "mt-4 grid gap-4", Outlet::<Route> {} }

                footer { class: "mt-4 text-center text-sm text-muted", p { "Proteus Audio Project" } }
            }
        }
    }
}

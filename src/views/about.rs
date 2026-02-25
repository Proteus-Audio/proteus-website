use crate::components::SectionPanel;
use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let paragraph_class = "mt-3 text-sm leading-7 text-muted md:text-base";
    let link_class = "underline decoration-[var(--primary)] decoration-1 underline-offset-2 hover:text-[var(--primary-deep)]";
    let card_class = "surface-card mt-4 p-4 md:p-5";
    let cta_class = "inline-flex items-center justify-center rounded-sm border border-[var(--primary)] px-3 py-2 text-sm font-semibold text-[var(--primary-deep)] transition hover:bg-[var(--primary)]/10";

    rsx! {
        SectionPanel {
            h2 { class: "text-3xl font-bold text-[var(--text)]", "About Proteus" }

            blockquote {
                class: "mt-4 rounded-r-sm border-l-4 border-[var(--analog)] bg-[#fffbef] px-4 py-3 text-sm text-[#5a4c25]",
                "“It’s possible that our grandchildren will look at us and say ‘You mean people used to listen to the same thing over and over again?’” - Brian Eno"
            }

            p {
                class: paragraph_class,
                "Proteus grew out of a train of thoughts inspired by a 2014 lecture by Dr. Andy Farnell at the University of Edinburgh which spoke, in part, about the distinction between fixed and performance mediums (ie film vs stage, album vs concert)."
            }

            p {
                class: paragraph_class,
                "Though, undoutably, much of the draw of performance art is owed to community and social connection, I think there’s a case to be made that some of the power of perfomance is in its subtle unpredictability. The Proteus Audio Project is an attempt to bring some of that unpredictability to recorded music."
            }

            p {
                class: paragraph_class,
                "The project explores a song format where multiple real takes of each part can be packaged together, then recombined at playback into a new but still intentional version of the same piece. That keeps the artistic integrity, but brings in a subtle (or not-subtle if the artist wished) level of unpredictability."
            }

            p {
                class: paragraph_class,
                "I started building in 2020 with simple proof-of-concept tooling, then moved through Flutter and Electron builds before landing on a Rust-focused stack and the current Proteus toolchain. Along the way, the format settled around packaged multi-part audio playback with room for metadata and playback guidance."
            }

            p {
                class: paragraph_class,
                "While the applications are now functional, there’s still plenty to do. If you want to follow along, keep an eye on the project repos and "
                a {
                    class: link_class,
                    href: "https://github.com/Proteus-Audio/proteus-author/issues",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "issues page"
                }
                a {
                    class: link_class,
                    href: "https://github.com/Proteus-Audio/proteus-player/issues",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "(s)"
                }
                ", or reach out at "
                a {
                    class: link_class,
                    href: "mailto:adam.thomas.howard@gmail.com",
                    "adam.thomas.howard@gmail.com"
                }
                "."
            }

            hr { class: "my-4 mt-8 border border-gray-200" }

            h3 { class: "mt-8 text-2xl font-bold text-[var(--text)]", "Proteus Author" }
            p {
                class: paragraph_class,
                "Proteus Author is the desktop app for building, organizing, and packaging Proteus projects into distributable .prot files."
            }
            div {
                class: "mt-4 grid gap-4 lg:grid-cols-[1.2fr_0.8fr]",
                    img {
                        class: "w-full rounded-sm bg-white object-cover",
                        src: "/images/prot-author-25-02-2026.webp",
                        alt: "Screenshot of the Proteus Author desktop application",
                    }
                div {
                    div {
                        class: "{card_class} flex flex-col gap-4",
                        div {
                            h4 { class: "text-lg font-semibold text-[var(--text)]", "Author and export .prot files." }
                            p {
                                class: "mt-2 text-sm leading-7 text-muted",
                                "A DAW inspired editor for managing track variants, track levels, mastering effects, and exporting projects into .prot structured files."
                            }
                        }
                        div { class: "mt-4",
                            Link {
                                to: Route::DownloadAuthor {},
                                class: cta_class,
                                "Download Proteus Author"
                            }
                        }
                    }
                }
            }

            hr { class: "my-4 mt-8 border border-gray-200" }

            h3 { class: "mt-8 text-2xl font-bold text-[var(--text)]", "Proteus Player" }
            p {
                class: paragraph_class,
                "Proteus Player is the listening app for playing .prot files and hearing a fresh take each time you press play."
            }
            div {
                class: "mt-4 grid gap-4 lg:grid-cols-[1.2fr_0.8fr]",
                div {
                    div {
                        class: "{card_class} flex flex-col gap-4",
                        div {
                            h4 { class: "text-lg font-semibold text-[var(--text)]", "Single-file player" }
                            p {
                                class: "mt-2 text-sm leading-7 text-muted",
                                "This application, inspired by Apple’s Quicktime Player, simply loads and plays single files. Stay tuned for a future application supporting a library-styled player."
                            }
                        }
                        div { class: "mt-4",
                            Link {
                                to: Route::DownloadPlayer {},
                                class: cta_class,
                                "Download Proteus Player"
                            }
                        }
                    }
                },
                img {
                    class: "w-full rounded-sm bg-white object-cover",
                    src: "/images/prot-player-25-02-2026.webp",
                    alt: "Screenshot of the Proteus Author desktop application",
                }
            }
        }
    }
}

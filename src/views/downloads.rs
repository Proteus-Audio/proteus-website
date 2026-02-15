use dioxus::prelude::*;

#[component]
pub fn Downloads() -> Element {
    rsx! {
        section {
            class: "content-panel",
            h2 { "Downloads" }
            p {
                "Release links will be finalized once packaging for each target is ready."
            }

            div {
                class: "download-grid",
                article {
                    class: "download-card",
                    h3 { "Proteus Author" }
                    p { "Desktop authoring environment for creating and exporting .prot projects." }
                    button { class: "btn btn-disabled", disabled: true, "Coming Soon" }
                }
                article {
                    class: "download-card",
                    h3 { "Proteus Player" }
                    p { "Dedicated player for opening and listening to .prot files." }
                    button { class: "btn btn-disabled", disabled: true, "Coming Soon" }
                }
                article {
                    class: "download-card",
                    h3 { "Proteus CLI" }
                    p { "Command-line utilities for parsing, validating, and automating project workflows." }
                    button { class: "btn btn-disabled", disabled: true, "Coming Soon" }
                }
            }

            p {
                class: "note",
                "Need immediate access? Add direct GitHub release URLs or installer links once they are available."
            }
        }
    }
}

use crate::components::SectionPanel;
use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let paragraph_class = "mt-3 text-sm leading-7 text-muted md:text-base";
    let link_class = "underline decoration-[var(--primary)] decoration-1 underline-offset-2 hover:text-[var(--primary-deep)]";

    rsx! {
        SectionPanel {
            h2 { class: "text-3xl font-bold text-[var(--text)]", "About Proteus" }

            blockquote {
                class: "mt-4 rounded-r-sm border-l-4 border-[var(--analog)] bg-[#fffbef] px-4 py-3 text-sm text-[#5a4c25]",
                "“It’s possible that our grandchildren will look at us and say ‘You mean people used to listen to the same thing over and over again?’” - Brian Eno"
            }

            p {
                class: paragraph_class,
                "I attended a lecture in 2014 by Dr. Andy Farnell on Procedural Audio who spoke, in part, about the distinction between fixed and performance mediums (ie film vs stage, album vs concert). Making note of the fact that while a theatre performance has a fixed structure and the story envokes a mood, it also adapts itself to the space and time of the specific performance."
            }

            p {
                class: paragraph_class,
                "Though, undoutably, much of the draw of performance art is owed to community and social connection, I think there’s a case to be made that some of the power of perfomance is in its subtle unpredictability."
            }

            p {
                class: paragraph_class,
                "While the world of popular cinematic storytelling is, at least in part, beginning to push itself out of a fixed format ("
                a {
                    class: link_class,
                    href: "https://www.npr.org/2018/12/28/680671691/black-mirror-bandersnatch-makes-you-choose-your-own-adventure",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    i { "Black Mirror: Bandersnatch" }
                }
                " / "
                a {
                    class: link_class,
                    href: "https://help.netflix.com/en/node/62526",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Neflix’s growing library of interactive content"
                }
                ") and the world of video gaming, which has long-touted interactive storytelling, is "
                a {
                    class: link_class,
                    href: "https://youtu.be/d8B1LNrBpqc",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "approaching cinematic realism"
                }
                ", popular recorded music is still very much fixed."
            }

            p {
                class: paragraph_class,
                "Procedural music itself is not a new thing, the video game and contemporary composition communities have been exploring it for a long while (Steve Reich’s "
                a {
                    class: link_class,
                    href: "https://www.npr.org/sections/deceptivecadence/2015/01/27/381575433/fifty-years-of-steve-reichs-its-gonna-rain",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    i { "It’s Gonna Rain" }
                }
                " was recorded in 1965). But, as of yet, examples of procedural music in the realm of song are sparse."
            }

            p {
                class: paragraph_class,
                "The, possibly obvious, solution that I would like to explore would be to record a song in such a way that you have some number (say 10) of each individual part (ie, 10 takes of the vocal, 10 of the drums, 10 of the guitar, etc). Then on play back, you choose a random selection of each part. On a simple song with 5 parts (Guitar, Vocals, Drums, Bass, Synth) this would yield 100,000 unique combinations."
            }

            p {
                class: paragraph_class,
                "Widespread internet accessibility and the popularity of streaming music could make this potentially very achievable."
            }

            p {
                class: paragraph_class,
                "My first proof of concept of this variable playback format ( "
                a {
                    class: link_class,
                    href: "https://multiplay-wnabuuzq2q-uc.a.run.app/?ref=ath",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "hosted here"
                }
                " ) used "
                a {
                    class: link_class,
                    href: "http://sox.sourceforge.net/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "SoX"
                }
                " to simply combine the parts of a short piece into a new random composite file. In early 2021, I started to work on expanding the idea out with two "
                a {
                    class: link_class,
                    href: "https://flutter.dev/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Flutter"
                }
                "-based desktop applications ( "
                a {
                    class: link_class,
                    href: "https://github.com/howardah/multiplay",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "here"
                }
                " & "
                a {
                    class: link_class,
                    href: "https://github.com/howardah/multiplay_mixer",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "here"
                }
                " ) which read and write "
                a {
                    class: link_class,
                    href: "https://www.matroska.org/index.html",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Matroska"
                }
                " Audio files. Using a streamable container file format like Matroska, it is possible to hold all the parts in one distinct package and stream different sets together as well has include additional data which can serve as a guide for how to process each part of the recording."
            }

            p {
                class: paragraph_class,
                "In mid-2022, I decided to replace the flutter applications with an "
                a {
                    class: link_class,
                    href: "https://www.electronjs.org/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "ElectronJS"
                }
                " application in order to make use of the flexibility of CSS styling and, at the same time, decided to name the project after the Greek sea-god "
                a {
                    class: link_class,
                    href: "https://en.wikipedia.org/wiki/Proteus",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Proteus"
                }
                " who represents mutability and is the root of the adjective ‘protean’."
            }

            p {
                class: paragraph_class,
                "Shortly after beginning to write the electron application, I realised that the resulting file size and performance of the build was far from ideal for a, relatively, simple application. I did some additional research and found "
                a {
                    class: link_class,
                    href: "https://tauri.app/",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Tauri"
                }
                " which offers nearly everything that I was looking for with electron but with "
                i { "significantly" }
                " improved performance. Tauri's perfomative Rust-based encouraged me to build out a Rust-based CLI for parsing and playing .prot files which in integrated into this project as well as the "
                a {
                    class: link_class,
                    href: "https://github.com/Proteus-Audio/proteus-player",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "Proteus Player"
                }
                " application."
            }

            p {
                class: paragraph_class,
                "There's still much to do with the project so, if you would like to follow along, you can keep tabs on this repo its "
                a {
                    class: link_class,
                    href: "https://github.com/Proteus-Audio/proteus-author/issues",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "issues page"
                }
                ". If you’d like talk about the idea, feel free to give me a shout at "
                a {
                    class: link_class,
                    href: "mailto:adam.thomas.howard@gmail.com",
                    "adam.thomas.howard@gmail.com"
                }
                "!"
            }
        }
    }
}

#![allow(non_snake_case)]
use std::path::Path;

use crate::{get_file_base64};
use dioxus::{prelude::*};
use once_cell::sync::Lazy;


fn svg_64(path: impl AsRef<Path>) -> String {
    format!("data:image/svg+xml;base64,{}", get_file_base64(path))
}
static DIOXUS_IMG: Lazy<String> = Lazy::new(|| svg_64("dioxus.svg"));
static TAILWIND_IMG: Lazy<String> = Lazy::new(|| svg_64("tailwind.svg"));
static GITHUB_IMG: Lazy<String> = Lazy::new(|| svg_64("github.svg"));

pub fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let dioxus_hover = use_state(cx, || false);
    let dioxus_hover_animation = || if **dioxus_hover { "animate-dioxus" } else { "" };
    let tailwind_hover = use_state(cx, || false);
    let tailwind_hover_animation = || {
        if **tailwind_hover {
            "animate-tailwind"
        } else {
            ""
        }
    };

    let rsx = rsx!(
        div {
            class: "flex flex-col h-screen items-center",
            a {
                class: "absolute right-0 p-8",
                href: "https://github.com/lyonsyonii/dioxus-tailwindcss",
                target: "_blank",
                img {
                    class: "w-8 sm:w-16",
                    src: "{*GITHUB_IMG}",
                }
            }
            div {
                class: "grid grid-cols-5 pb-16 px-8 sm:mt-60 mt-28 font-bold text-3xl sm:text-5xl gap-3",
                a {
                    class: "col-span-2 place-self-end mr-4 sm:mr-6 {dioxus_hover_animation()}",
                    href: "https://dioxuslabs.com",
                    target: "_blank",
                    onmouseenter: move |_| dioxus_hover.set(true),
                    onanimationend: move |_| dioxus_hover.set(false),
                    img {
                        class: "h-28 sm:h-44 hover:drop-shadow-blue transition-shadow",
                        src: "{*DIOXUS_IMG}",
                    },
                }
                a {
                    class: "col-span-2 col-start-4 place-self-center {tailwind_hover_animation()}",
                    href: "https://tailwindcss.com",
                    target: "_blank",
                    onmouseenter: move |_| tailwind_hover.set(true),
                    onanimationend: move |_| tailwind_hover.set(false),
                    img {
                        class: "w-28 sm:w-44 hover:drop-shadow-blue transition-shadow",
                        src: "{*TAILWIND_IMG}"
                    }
                }
                h1 {
                    class: "col-span-2 place-self-end",
                    "Dioxus"
                },
                h1 {
                    class: "place-self-center",
                    "❤️"
                },
                h1 {
                    class: "col-span-2",
                    "Tailwind"
                },
            }
            button {
                class: "bg-gray-200 px-4 py-2 rounded-lg border border-white hover:border-indigo-500 active:scale-95 transition-all",
                onclick: move |_| count += 1,
                "count is {count}"
            }
            div {
                class: "absolute sm:relative bottom-8 sm:bottom-auto",
                p {
                    class: "pt-4 px-8",
                    "Edit " Code {"src/app.rs"} " and run " Code {"dioxus serve"} " to test Hot Reload"
                }
                p {
                    class: "text-slate-500 pt-8 sm:pt-16 px-8",
                    "Click on the Dioxus and Tailwind logos to learn more"
                }
            }

        }

    );

    cx.render(rsx)
}

#[inline_props]
fn Code<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx! {
        code {
            class: "bg-gray-100 text-gray-900 rounded px-1 py-0.5 font-mono text-sm",
            children
        }
    })
}

#[inline(always)]
fn static_svg(base64: &Lazy<String>) -> String {
    format!(",{}", **base64)
}
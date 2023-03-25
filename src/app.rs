#![allow(non_snake_case)]
use dioxus::prelude::*;
use macros::include_url_encoded;

const DIOXUS_IMG: &'static str = include_url_encoded!("assets/dioxus.svg");
const TAILWIND_IMG: &'static str = include_url_encoded!("assets/tailwind.svg");

pub fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let dioxus_hover = use_state(cx, || false);
    let dioxus_hover_animation = || if_then_else(dioxus_hover, "animate-dioxus", "");
    let tailwind_hover = use_state(cx, || false);
    let tailwind_hover_animation = || if_then_else(tailwind_hover, "animate-tailwind", "");

    let rsx = rsx!(
        div {
            class: "flex flex-col h-screen items-center",
            div {
                class: "grid grid-cols-5 pb-16 mt-60 font-bold text-5xl gap-3",

                a {
                    class: "col-span-2 place-self-end mr-6 {dioxus_hover_animation()}",
                    href: "https://dioxuslabs.com",
                    target: "_blank",
                    onmouseenter: move |_| dioxus_hover.set(true),
                    onanimationend: move |_| dioxus_hover.set(false),
                    img {
                        class: "h-44 min-h-44 hover:drop-shadow-blue transition-shadow",
                        src: "data:image/svg+xml;utf8,{DIOXUS_IMG}",
                    },
                }
                a {
                    class: "col-span-2 col-start-4 place-self-center {tailwind_hover_animation()}",
                    href: "https://tailwindcss.com",
                    target: "_blank",
                    onmouseenter: move |_| tailwind_hover.set(true),
                    onanimationend: move |_| tailwind_hover.set(false),
                    img {
                        class: "w-44 min-w-44 hover:drop-shadow-blue transition-shadow",
                        src: "data:image/svg+xml;utf8,{TAILWIND_IMG}"
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
            p {
                class: "pt-4",
                "Edit " Code {"src/app.rs"} " and run " Code {"dioxus serve"} " to test Hot Reload"
            }
            p {
                class: "text-slate-500 pt-16",
                "Click on the Dioxus and Tailwind logos to learn more"
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

fn if_then_else<T>(r#if: &UseState<bool>, then: T, r#else: T) -> T {
    if *r#if.get() {
        then
    } else {
        r#else
    }
}

// TODO! Integrate with https://github.com/Demonthos/tiny-dioxus
#![allow(non_snake_case)]
use dioxus::{prelude::*};

pub fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);

    let rsx = rsx!(
        div {
            class: "flex flex-col h-screen items-center",
            div {
                class: "grid grid-cols-2 w-1/4 min-w-fit place-items-center mt-60",
                a {
                    class: "hover:animate-dioxus",
                    href: "https://dioxuslabs.com",
                    target: "_blank",
                    img {
                        class: "h-44 min-h-44 hover:drop-shadow-blue transition-shadow",
                        src: "dioxus.svg",
                    }
                }
                a {
                    class: "hover:animate-tailwind",
                    href: "https://tailwindcss.com",
                    target: "_blank",
                    img {
                        class: "w-44 min-w-44 hover:drop-shadow-blue transition-shadow",
                        src: "tailwind.svg"
                    }
                }
            }
            h1 {
                class: "py-16 font-bold text-5xl",
                "Dioxus + Tailwind"
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
pub fn Code<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
    cx.render(rsx! {
        code {
            class: "bg-gray-100 text-gray-900 rounded px-1 py-0.5 font-mono text-sm",
            children
        }
    })
}
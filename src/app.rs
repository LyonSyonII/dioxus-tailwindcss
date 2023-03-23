#![allow(non_snake_case)]
use dioxus::{prelude::*};

pub fn App(cx: Scope) -> Element {
    let rsx = rsx!(
        div {
            class: "h-screen grid grid-cols-3 items-center justify-evenly gap-8",
            
            img {
                class: "w-1/2 text-center",
                src: "dioxus.svg"
            }
            img {
                class: "w-1/3",
                src: "heart.svg"
            }
            img {
                class: "w-2/3",
                src: "tailwind.svg"
            }
        }
    );
    
    cx.render(rsx)
}
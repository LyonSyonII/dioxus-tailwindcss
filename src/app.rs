#![allow(non_snake_case)]
use dioxus::{prelude::*};

pub fn App(cx: Scope) -> Element {
    let rsx = rsx!(
        div {
            class: "w-2/3 h-screen grid grid-cols-1 md:grid-cols-3 items-center justify-evenly mx-auto animate-wave",
            
            img { 
                class: "w-1/2 justify-self-center animate-wave",
                src: "dioxus.svg"
            }
            
            img {
                class: "w-1/3 justify-self-center",
                src: "heart.svg"
            }
            img {
                class: "w-2/3 justify-self-center animate-wave",
                src: "tailwind.svg"
            }
        }
    );
    
    cx.render(rsx)
}
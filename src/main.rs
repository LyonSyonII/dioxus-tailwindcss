#![allow(non_snake_case)]
use dioxus::{prelude::*};

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(Root);
  
    #[cfg(any(windows, unix))]
    dioxus_desktop::launch(Root);
}

fn Root(cx: Scope) -> Element {
    let rsx = rsx!(
        style { include_str!("./index.css") }
        App {}
    );
    
    cx.render(rsx)
}

fn App(cx: Scope) -> Element {
    let rsx = rsx!(
        h1 {
            class: "transform font-bold text-5xl text-center underline p-4 hover:scale-150 transition-transform",
            "Title"
        }
        div {
            div {
                div {
                    class: "bg-pink-500",
                    "I'm pinky"
                }
            }
        }
    );
    
    cx.render(rsx)
}
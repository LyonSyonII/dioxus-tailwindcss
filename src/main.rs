#![allow(non_snake_case)]
use dioxus::{prelude::*};

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(Root);
  
    #[cfg(any(windows, unix))]
    dioxus_desktop::launch(Root)
    // dioxus_tui::launch(Root)
}

fn Root(cx: Scope) -> Element {
    let rsx = rsx!(
        head {
            // TODO: https://github.com/DioxusLabs/dioxus/blob/540e785d8bd18b177b08c1bbf72d17a2667c312b/examples/tailwind.rs
            link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@^2.0/dist/tailwind.min.css" }
        }
        body {
            App {}
        }
    );
    
    cx.render(rsx)
}

fn App(cx: Scope) -> Element {
    let rsx = rsx!(
        h1 {
            class: "transform font-bold text-5xl text-center underline p-4 hover:scale-150 transition-transform",
            "Title"
        }
    );
    
    cx.render(rsx)
}
#![allow(non_snake_case)]
use dioxus::prelude::*;

mod app;
use app::App;

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

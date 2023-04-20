#![allow(non_snake_case)]

use dioxus::prelude::*;

pub fn App(cx: Scope) -> Element {
    let rsx = rsx!(
        "Your code here"
    );
    cx.render(rsx)
}

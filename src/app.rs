#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::public; // Default path for public files is "$crate/public", change it in the "public.rs" file

pub fn App(cx: Scope) -> Element {
    let rsx = rsx!(
        "Your code here"
    );
    cx.render(rsx)
}

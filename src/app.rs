#![allow(non_snake_case)]

use crate::public::StaticPublic;
use dioxus::prelude::*;

static DIOXUS: public::Svg = public::StaticPublic::svg("dioxus.svg");

pub fn App(cx: Scope) -> Element {
    let rsx = rsx!("{DIOXUS}");
    cx.render(rsx)
}

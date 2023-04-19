#![allow(non_snake_case)]
use std::path::{Path};

use base64::Engine;
use dioxus::prelude::*;
use include_dir::{include_dir, Dir};

mod app;
use app::App;

static PUBLIC: Dir = include_dir!("public");

fn get_file(path: impl AsRef<Path>) -> &'static str {
    let contents = PUBLIC.get_file(path).unwrap().contents();
    std::str::from_utf8(contents).unwrap()
}

static BASE64: base64::engine::GeneralPurpose = base64::prelude::BASE64_URL_SAFE_NO_PAD;

fn get_file_base64<P: AsRef<Path>>(path: P) -> String {
    let file = get_file(path);
    BASE64.encode(file)
}

fn main() {
    dioxus_desktop::launch_cfg(
        Root,
        dioxus_desktop::Config::new()
            .with_custom_head(format!(r#"<style>{}</style>"#, get_file("tailwind.css"))),
    );
}

fn Root(cx: Scope) -> Element {
    let rsx = rsx!(App {});

    cx.render(rsx)
}
#![allow(non_snake_case)]
use std::ops::Deref;

use dioxus::prelude::*;

mod app;
mod public;
use image::EncodableLayout;

fn main() {
    let title = env!("CARGO_PKG_NAME");
    let tailwind = public::get_file("tailwind.css");
    let icon = || {
        use image::io::Reader as ImageReader;
        use image::ImageFormat;
        use std::io::Cursor;

        let mut reader = ImageReader::new(Cursor::new(try_get_file_bytes("icon.png")?));
        reader.set_format(ImageFormat::Png);
        let icon = reader.decode().unwrap();
        let bytes = icon.as_rgba8().unwrap();
        Some(
            dioxus_desktop::tao::window::Icon::from_rgba(
                bytes.to_vec(),
                icon.width(),
                icon.height(),
            )
            .unwrap(),
        )
    };

    let head = format!(
        r#"
        <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <meta charset="UTF-8" />
        <link data-trunk rel="rust" data-wasm-opt="z" />
        <style>{tailwind}</style>
        "#
    );

    // TODO! Does not work at the moment, same with `dioxus_desktop::Config::new().with_icon()`
    let window = dioxus_desktop::WindowBuilder::new()
        .with_title(title)
        .with_window_icon(icon());

    dioxus_desktop::launch_cfg(
        Root,
        dioxus_desktop::Config::new()
            .with_custom_head(head)
            .with_window(window),
    );
}

fn Root(cx: Scope) -> Element {
    let rsx = rsx!(app::App {});

    cx.render(rsx)
}

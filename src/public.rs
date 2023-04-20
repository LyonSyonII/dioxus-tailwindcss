use std::{
    fmt::{Display, Debug},
    ops::{Deref, DerefMut},
    path::Path, collections::HashMap,
};

use base64::Engine;
use dioxus::html::svg;
use include_dir::{include_dir, Dir};
use once_cell::sync::OnceCell;

static PUBLIC: Dir = include_dir!("public");
static BASE64: base64::engine::GeneralPurpose = base64::prelude::BASE64_URL_SAFE_NO_PAD;

/// Get a file from the `public` directory.
///
/// # Panics
/// Panics if the path is not valid
pub fn get_file(path: impl AsRef<Path>) -> &'static str {
    let contents = PUBLIC.get_file(path).unwrap().contents();
    std::str::from_utf8(contents).unwrap()
}

pub fn get_file_bytes(path: impl AsRef<Path>) -> &'static [u8] {
    PUBLIC.get_file(path).unwrap().contents()
}

pub fn try_get_file_bytes(path: impl AsRef<Path>) -> Option<&'static [u8]> {
    Some(PUBLIC.get_file(path)?.contents())
}

pub fn get_file_base64<P: AsRef<Path>>(path: P) -> String {
    let file = get_file(path);
    BASE64.encode(file)
}

pub fn svg_64(path: impl AsRef<Path>) -> String {
    format!("data:image/svg+xml;base64,{}", get_file_base64(path))
}

/// Static SVG in base64 form located in the `public/` directory.
///
/// Place it inside an `img { src }` tag to display it.
///
/// # Panics
/// Panics if the file is not in the `public/` directory or if it's not valid utf8.
///
/// # Example
/// ```
/// use dioxus::prelude::*;
/// use crate::public::Svg;
///
/// // Image located in `public/dioxus.svg`
/// static IMAGE: Svg = Svg::new("dioxus.svg");
///
/// pub fn App(cx: Scope) -> Element {
///     let rsx = rsx!(
///         img {
///             src: "{IMAGE}"
///         }
///     );
///     cx.render(rsx)
/// }
/// ```
pub struct Svg {
    content: OnceCell<String>,
    path: &'static str,
}

impl Svg {
    pub const fn new(path: &'static str) -> Svg {
        Svg {
            content: OnceCell::new(),
            path,
        }
    }
}

impl Deref for Svg {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self.content.get_or_init(|| svg_64(self.path))
    }
}

impl Display for Svg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.deref())
    }
}

pub struct StaticPublic<T, F> {
    content: OnceCell<T>,
    path: &'static str,
    init: F,
}

impl<T, F> StaticPublic<T, F> where F: Fn(&str) -> T {
    const fn new(path: &'static str, init: F) -> StaticPublic<T, F> {
        StaticPublic { content: OnceCell::new(), path, init }
    }
}

impl<T, F> Deref for StaticPublic<T, F> where F: Fn(&str) -> T {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        self.content.get_or_init(|| (self.init)(self.path))
    }
}

impl<T, F> Display for StaticPublic<T, F> where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl<T, F> Debug for StaticPublic<T, F> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticPublic").field("path", &self.path).field("content", self.deref()).finish()
    }
}
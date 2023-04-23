#![allow(dead_code)]
#![forbid(missing_docs)]


//! Saves all the files on the `public` directory at compile time and allows to access them at runtime or save to a `static` variable.
//! 
//! # Example
//! 
//! ```
//! // File located at "public/icon.svg"
//! static LOGO: public::Svg = public::svg("icon.svg");
//! 
//! fn main() {
//!     // File located at "public/data.txt"
//!     let runtime: &'static str = public::get_file("data.txt");
//! }
//! ```

use std::{
    fmt::{Debug, Display},
    ops::Deref,
    path::Path,
};

use base64::Engine;
use include_dir::{include_dir, Dir};
use once_cell::sync::{OnceCell};

static PUBLIC: Dir = include_dir!("public"); // CHANGE THIS TO YOUR PREFERRED LOCATION
static BASE64: base64::engine::GeneralPurpose = base64::prelude::BASE64_URL_SAFE_NO_PAD;



/// Get a file from the `public` directory as `&'static str`.
///
/// # Panics
/// Panics if the path is not valid.
pub fn get_file(path: impl AsRef<Path>) -> &'static str {
    std::str::from_utf8(get_file_bytes(path)).unwrap()
}

/// Get a file from the `public` directory as `&'static str`.
/// 
/// Returns `None` if the path is not valid or the file contains non-utf8 characters.
pub fn try_get_file(path: impl AsRef<Path>) -> Option<&'static str> {
    std::str::from_utf8(try_get_file_bytes(path)?).ok()
}

/// Get a file from the `public` directory as a byte slice.
///
/// # Panics
/// Panics if the path is not valid.
pub fn get_file_bytes(path: impl AsRef<Path>) -> &'static [u8] {
    try_get_file_bytes(path).unwrap()
}

/// Get a file from the `public` directory as a byte slice.
///
/// Returns `None` if the path is not valid.
pub fn try_get_file_bytes(path: impl AsRef<Path>) -> Option<&'static [u8]> {
    Some(PUBLIC.get_file(path)?.contents())
}

/// Get a file from the `public` directory as a Base64-encoded `String`.
///
/// # Panics
/// Panics if the path is not valid.
pub fn get_file_base64<P: AsRef<Path>>(path: P) -> String {
    BASE64.encode(get_file(path))
}

/// Wrapper around the `public` folder access.
///
/// Should not be used by itself, access only with the `public::asset` methods.
///
/// # Example
/// ```
/// static IMAGE: public::Svg = public::svg("images/cat.svg");
/// ```
pub struct StaticPublic<T> {
    content: OnceCell<T>,
    path: &'static str,
    init: fn(&str) -> T,
}

impl<T> StaticPublic<T> {
    const fn new(path: &'static str, init: fn(&str) -> T) -> StaticPublic<T> {
        StaticPublic {
            content: OnceCell::new(),
            path,
            init,
        }
    }
}

impl<T> Deref for StaticPublic<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.content.get_or_init(|| (self.init)(self.path))
    }
}

impl<T: Display> Display for StaticPublic<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl<T: Debug> Debug for StaticPublic<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StaticPublic")
            .field("path", &self.path)
            .field("content", self.deref())
            .finish()
    }
}

// StaticPublic implementations

fn fmt_b64_img(format: &'static str, path: &str) -> String {
    format!("data:image/{};base64,{}", format, get_file_base64(path))
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
/// use crate::public;
///
/// // Image located in `public/dioxus.svg`
/// static IMAGE: public::Svg = public::svg("dioxus.svg");
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
pub const fn svg(path: &'static str) -> Svg {
    StaticPublic::new(path, |path| fmt_b64_img("svg+xml", path))
}
/// See [`public::svg`](crate::public::svg) for usage information.
pub type Svg = StaticPublic<String>;

/// Static PNG in base64 form located in the `public/` directory.
///
/// Place it inside an `img { src }` tag to display it.
///
/// # Panics
/// Panics if the file is not in the `public/` directory or if it's not valid utf8.
///
/// # Example
/// ```
/// use dioxus::prelude::*;
/// use crate::public;
///
/// // Image located in `public/dioxus.svg`
/// static IMAGE: public::Png = public::png("dioxus.png");
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
pub const fn png(path: &'static str) -> Png {
    StaticPublic::new(path, |path| fmt_b64_img("png", path))
}

/// See [`public::png`](crate::public::png) for usage information.
pub type Png = StaticPublic<String>;

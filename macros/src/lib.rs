use percent_encoding::{NON_ALPHANUMERIC, utf8_percent_encode};
use proc_macro::TokenStream;


/// Includes a UTF-8 encoded file as a URL-encoded string.
///
/// The file is located **relative to the Crate's root** (not the current file like `include_str!`).
/// 
/// The provided path is interpreted in a platform-specific
/// way at compile time. So, for instance, an invocation with a Windows path
/// containing backslashes `\` would not compile correctly on Unix.
///
/// This macro will yield an expression of type `&'static str` which is the
/// contents of the file.
///
/// # Examples
/// ```ignore (cannot-doctest-external-file-dependency)
/// fn main() {
///     // File contains: "example text :)"
///     let my_str = include_encoded!("example.txt"); 
///     assert_eq!(my_str, "example%20text%20%3A%29");
/// }
/// ```
#[proc_macro]
pub fn include_url_encoded(input: TokenStream) -> TokenStream {
    let input = input.to_string().replace("\"", "");
    let input = std::fs::read_to_string(input).unwrap();
    let output = url_encode(&input);
    format!(" \"{output}\" ").parse().unwrap()
}


fn url_encode(svg_data: &str) -> String {
    utf8_percent_encode(svg_data, NON_ALPHANUMERIC).to_string()
}
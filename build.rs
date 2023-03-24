fn main() {
    // println!("Compiling tailwindCSS...");
    // panic!("{:?}", std::env::var("PATH"));
    let mut tailwind = std::process::Command::new("tailwind");
    tailwind.args([
        "-c",
        "tailwind.config.js",
        "-o",
        "src/index.css",
        "--minify",
    ]);
    tailwind.env("NODE_ENV", "production");
    tailwind.spawn().unwrap();
}

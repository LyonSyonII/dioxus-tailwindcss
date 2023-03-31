fn main() {
    // println!("Compiling tailwindCSS...");
    // panic!("{:?}", std::env::var("PATH"));
    let mut tailwind = std::process::Command::new("tailwind");
    tailwind.args([
        "-i",
        "src/index.css",
        "-c",
        "tailwind.config.js",
        "-o",
        "public/tailwind.css",
        "--minify",
    ]);
    tailwind.env("NODE_ENV", "production");
    tailwind.spawn().unwrap();
}

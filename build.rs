fn main() {
    // Install required packages
    let toolchain = install_packages();
    
    // Compile TailwindCSS .css file
    std::process::Command::new(toolchain)
        .args([
            "tailwind",
            "-i",
            "src/index.css",
            "-c",
            "tailwind.config.js",
            "-o",
            "public/tailwind.css",
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .unwrap();
}


/// Installs required packages and selects toolchain to use.
/// 
/// It will prioritize `yarn` over `npm`.
/// 
/// # Panic
/// Will panic if none of the toolchains is installed.
fn install_packages() -> &'static str {
    if std::process::Command::new("yarn").arg("install").spawn().is_ok() {
        return "yarn"
    }
    
    match std::process::Command::new("node").arg("install").spawn() {
        Ok(_) => "npx",
        Err(e) => panic!("ERROR: Npm or Yarn installation is needed.\n{e}"),
    }
}
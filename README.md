# Dioxus + TailwindCSS
Example and Template for the Rust Dioxus desktop/web framework.

Demo: https://garriga.dev/dioxus-tailwindcss

## Common Dependencies
Install tailwind-cli:
```bash
yarn global add tailwindcss
```
Remember to have it included in the PATH or you'll recieve an error when building the crate.

## Desktop
### Dependencies
#### Windows

Windows Desktop apps depend on WebView2 – a library that should be installed in all modern Windows distributions. If you have Edge installed, then Dioxus will work fine. If you *don't* have Webview2, [then you can install it through Microsoft](https://developer.microsoft.com/en-us/microsoft-edge/webview2/). MS provides 3 options:

1. A tiny "evergreen" *bootstrapper* that fetches an installer from Microsoft's CDN
2. A tiny *installer* that fetches Webview2 from Microsoft's CDN
3. A statically linked version of Webview2 in your final binary for offline users

For development purposes, use Option 1.

---

#### Linux

Webview Linux apps require WebkitGtk. When distributing, this can be part of your dependency tree in your `.rpm` or `.deb`. However, likely, your users will already have WebkitGtk.

```bash
sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libappindicator3-dev
```

When using Debian/bullseye `libappindicator3-dev` is no longer available but replaced by `libayatana-appindicator3-dev`.

```bash
# on Debian/bullseye use:
sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev
```

If you run into issues, make sure you have all the basics installed, as outlined in the [Tauri docs](https://tauri.studio/v1/guides/getting-started/prerequisites#setting-up-linux).

---

### Running
```bash
cargo run
```

## Web
```bash
cargo install dioxus-cli
```
Install the required target:

```bash
rustup target add wasm32-unknown-unknown
```
### Serve/Dev
```bash
dioxus serve
```
### Build
```bash
dioxus build --release
``` 

## Troubleshooting
Problems I encountered while making this demo.

* ### `public` files are not loading in Desktop builds  
    In Desktop builds, the path of the assets is based on the root of the crate.  
    So, for example, to load a file located on "$CRATE_ROOT/public/example.svg" you'd write:  
        * Web: `src: "example.svg"`  
        * Desktop: `src: "public/example.svg"`
    
    In the example I've used conditional compilation to change the path depending on the build (look at `src/app.rs -> const DIOXUS_IMG`).

* ### Drop Shadows look broken when the logos are animated in Desktop builds
    I'm aware of it, but don't know the solution unfortunately, in web it works fine though.

* ### Why not use the CDN instead of Tailwind CLI?
    The CDN recommended by the official examples of Dioxus is outdated, and the newest version of Tailwind does not support that kind of usage.

    Also, it's not recommended to use in production, as it inserts ALL css classes into your html.

    The workaround I've used is based on the instructions of the [`tailwind.rs`](https://dev.to/arctic_hen7/how-to-set-up-tailwind-css-with-yew-and-trunk-il9) example (`dioxus/examples/tailwind.rs`).
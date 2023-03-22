# Dioxus + TailwindCSS
Template for the Rust Dioxus desktop/web framework.

## Desktop
### Dependencies
---
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
dioxus build
``` 

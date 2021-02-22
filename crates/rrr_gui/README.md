# rrr_gui

[![Crate Docs Main]][docs main]

[crate docs main]: https://img.shields.io/badge/docs-main-4e73a5
[docs main]: https://flashflashrevolution.github.io/rrr/api/rrr_gui

Web GUI for Rust Rust Revolution.

## Build and Run

```zsh
rustup install nightly
rustup target add wasm32-unknown-unknown
cargo install trunk wasm-bindgen-cli
# pwd : ./crates/rrr_gui
trunk serve
```

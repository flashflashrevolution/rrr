# Rust Rust Revolution

[![Build Status]][actions]
[![Docs]][mdbook]
[![Chat]][discord]
[![rustc](https://img.shields.io/badge/rustc-1.65.0-lightgray.svg?style=round)](https://www.rust-lang.org/)
[![codecov](https://codecov.io/gh/flashflashrevolution/rrr/branch/main/graph/badge.svg?token=2V0ZGQ04IK)](https://codecov.io/gh/flashflashrevolution/rrr)
[![loc](https://tokei.rs/b1/github/flashflashrevolution/rrr)](https://github.com/flashflashrevolution/rrr/)

[actions]: https://github.com/flashflashrevolution/rrr/actions?query=branch%3Amain
[build status]: https://img.shields.io/github/workflow/status/flashflashrevolution/rrr/CI/main?logo=github
[chat]: https://img.shields.io/discord/196381154880782336?logo=discord
[discord]: https://discord.gg/ffr
[docs]: https://img.shields.io/badge/docs-book-blue?logo=gitbook
[mdbook]: https://rrr.flashflashrevolution.com/book/

---

## Development Environment

> Important steps to take before you'll be able to contribute to this project.

1. Install Visual Studio Code.
2. Install all of the recommended extensions.
3. Install rust toolchain:

   > ```sh
   > rustup toolchain install stable
   > rustup target add wasm32-unknown-unknown --toolchain stable
   > rustup component add llvm-tools-preview --toolchain stable
   > ```

4. Install the following cargo applications:

   > ```sh
   > cargo install cargo-watch
   > cargo install miniserve --locked
   > cargo install -f wasm-bindgen-cli
   > ```

> At this point you should be good to go.
> See [Developing](#developing) for launching, debugging and creation.

Note: If no tasks start, just start the miniserve task manually and then trash the task.
VSC will alert you of the available tasks and start them from then on automatically.

---

## Developing

> Visual Studio Code will automatically build and serve the web version of the game.

- Hitting F5 will run the game.
- Check the `Miniserve` terminal tab for the URL to view the game in the browser.

---

## License

Copyright © 2022 Zageron ([@zageron](https://twitter.com/zageron)), Fission

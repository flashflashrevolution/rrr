# Rust Rust Revolution

[![Build Status]][actions]
[![Docs]][mdbook]
[![Chat]][discord]
[![License]][license file]
[![rustc](https://img.shields.io/badge/rustc-1.61-lightgray.svg?style=round)](https://www.rust-lang.org/)
[![codecov](https://codecov.io/gh/flashflashrevolution/rrr/branch/main/graph/badge.svg?token=2V0ZGQ04IK)](https://codecov.io/gh/flashflashrevolution/rrr)

[actions]: https://github.com/flashflashrevolution/rrr/actions?query=branch%3Amain
[build status]: https://img.shields.io/github/workflow/status/flashflashrevolution/rrr/CI/main?logo=github
[chat]: https://img.shields.io/discord/196381154880782336?logo=discord
[discord]: https://discord.gg/ffr
[docs]: https://img.shields.io/badge/docs-book-blue?logo=gitbook
[license]: https://img.shields.io/github/license/flashflashrevolution/rrr?color=blue
[license file]: LICENSE
[mdbook]: https://flashflashrevolution.github.io/rrr/book

---

## Development Environment

> Important steps to take before you'll be able to contribute to this project.

1. Install Visual Studio Code.
2. Install all of the recommended extensions.
3. Install stable rust:
   > `rustup toolchain install stable`
4. Install the wasm target:
   > `rustup target add wasm32-unknown-unknown --toolchain stable`
5. Install llvm tools:
   > `rustup component add llvm-tools-preview --toolchain stable`
6. Install the following cargo applications:
   1. > `cargo install cargo-watch`
   2. > `cargo install miniserve --locked`

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
# Direct raylib bindings for Rust
[![Crates.io](https://img.shields.io/crates/v/raylib-ffi)](https://crates.io/crates/raylib-ffi)
[![Docs.rs](https://docs.rs/raylib-ffi/badge.svg)](https://docs.rs/raylib-ffi)
[![Build](https://github.com/Ewpratten/raylib-ffi/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/raylib-ffi/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/raylib-ffi/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/raylib-ffi/actions/workflows/clippy.yml)

`raylib-ffi` aims to provide a "no frills" direct binding to [raylib](https://www.raylib.com/) for rust developers. Unlike the [`raylib-rs` project](https://github.com/deltaphc/raylib-rs), this library trades high-level abstractions for staying truly up to date with upstream raylib (instead of a whole major version behind).

## Dependencies

Fedora:

```sh
dnf install clang-devel alsa-lib-devel mesa-libGL-devel libX11-devel libXrandr-devel libXi-devel libXcursor-devel libXinerama-devel libatomic cmake
```

## Verifying your build

`raylib-ffi` bundles a rust version of the example project from raylib proper. To verify your build of this library worked, run:

```sh
cargo run --example basic
```

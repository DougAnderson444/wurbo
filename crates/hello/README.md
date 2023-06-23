# WIT Component

Demo wasm component for the rollup plugin.

## Build Options

A. From _this_ directory, run:

`cargo component build --release`

B. or by package name:

`cargo component build --package hello --release`

C. or [watch](https://github.com/watchexec/cargo-watch) for changes:

`cargo watch -x 'component watch --release'`

## WASI Platform Use

The `src/lib.rs` will be built into a wasm component and placed in the `target/wasm32-wasi/release` directory.

`target/wasm32-wasi/release/hello.wasm` is the wasm file to use.

### Browser Use

Note: if any of the builds above are for browsers only, add the `--target wasm32-unknown-unknown` flag, such as:

`cargo component build--release --target wasm32-unknown-unknown`

This places the wasm file in the `target/wasm32-unknown-unknown/release` directory instead.

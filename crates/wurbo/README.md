# Wurbo-rs

The Experimental Rust crate for the [Wurbo framework](https://github.com/DougAnderson444/wurbo).

The purpose of this crate is to export a macro that builds the reactivity for you.

This enables you to make your WIT Wasm Component a reactive UI with minimal API surface.

The macro is `generate_reactivity! { ... }` which takes your [`render`](https://crates.io/crates/render) components and generates the `reactivity` interface for you.

## Publish 

Publish to crates.io

```bash
cargo publish
```


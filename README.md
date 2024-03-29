# Wurbo

### Because your user interface should be trustless, too.

Experimental Proof of concept web framework using [wasm components](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md). All input and UI generation is done in Wasm, so there cannot be "call home" vulnerabilities over your data inputs. This means you can use guest Apps (wasm components) in a trustless fashion in your host app.

Wurbo? Like [Turbo](https://github.com/hotwired/turbo), but using [Wasm Components](https://github.com/WebAssembly/component-model). Kind of like [Elm](https://guide.elm-lang.org/architecture/) too? Render HTML from wasm ui components.

## Short Video Hype 

[![Wurbo Demo](https://i.ytimg.com/vi/x2ooLUTYuQk/oar2.jpg?sqp=-oaymwEaCN0CENAFSFXyq4qpAwwIARUAAIhCcAHAAQY=&rs=AOn4CLAyYw1c2XvGjZwgVg1RdG_mjE7s9Q)](https://www.youtube.com/shorts/x2ooLUTYuQk)

## Examples

- Example Guest component is [`examples/vowels`](./examples/vowels/src/lib.rs)
- Example Forms component is [`examples/forms`](./examples/forms/src/lib.rs)
- Example Host app is [`src/routes/+page.svelte`](./src/routes/+page.svelte)

To run the examples locally in the browser, you can use the following commands:

```bash
# build-component:
  cargo component build --workspace

# prev: build-component
  npm run build && npm run preview -- --open
```

or using [just](https:://just.systems): `just prev`

## Demo

The example is demonstrated at [https://douganderson444.github.io/wurbo](https://douganderson444.github.io/wurbo/)

## Use Steps

1. In your WIT file:
- declare interface named `reactivity` with functions `render` and `activate`
- a separate interface (named anything you like, such as `imports`) with function `addeventlistener`

2. In the Rust:
- create your new [Wasm Component](https://github.com/bytecodealliance/cargo-component) using `cargo component new --reactor <name>`
- ensure you add [`render` crate](https://crates.io/crates/render) to your `Cargo.toml` dependencies
- `use` the `wurbo` crate's macro to implement the `reactivity` interface for `reactivity::Guest`. 
- build your own `Page` parent component and `Input` / `Output` components as you like for your user interface.

3. In JavaScript: 
- `load` the wasm bytes + importables into an ES module (called it `mod`) using `rollup-plugin-wit-component`
- `mod.reactivity.render(args)` uses that module to load the initial data.
- `wurbo.listen(mod)` listens for events from the component
- call `mod.reactivity.activate()` once the DOM has loaded, to start listening for change events.

Table Summary:

| Step | WIT | Rust | JavaScript |
| --- | --- | --- | --- |
| 1 | declare interface `imports` | pass `imports` to `wurbo` macro | implement `imports` as JS code then pass stringified code to `rollup-plugin-wit-component` fn `load` via `importables` |
| 2 | declare interface `reactivity` | implement interface `reactvity` using the macro and your [`render` crate](https://crates.io/crates/render) Components | load `reactivity` ES module using `rollup-plugin-wit-component` |
| 3 | Ready. | call `cargo component build --release` | call `mod.reactivity.render(args)` to get rendered HTML |
| 4 | Ready. | Ready. | call `wurbo.listen(mod)` to set up listeners |
| 5 | Ready. | Ready. | When DOM has loaded the rendered HTML, then call `mod.reactivity.activate()` to actually listen |

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

Compile the wasm component, see [README.md](./crates/vowels/README.md) for more details.

```bash
npm run build

# or start the server and open the app in a new browser tab
npm run preview -- --open
```

## Building

To build the code (dev mode does not work due to how Vite handles wasm):

```bash
cargo component build --workspace --release
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

## Publishing

To publish to npm:

```bash
npm run package
npm publish
```

To publish to crates.io:

```bash
cargo publish -p wurbo
```

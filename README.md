# Wurbo

Proof of concept web framework using [wasm components](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Explainer.md). All input and UI generation is done in Wasm, so there cannot be "call home" vulnerabilities over your data inputs. This means you can use guest Apps (wasm components) in a trustless fashion in your host app.

Wurbo? Like [Turbo](https://github.com/hotwired/turbo), but using [Wasm Components](https://github.com/WebAssembly/component-model). Kind of like [Elm](https://guide.elm-lang.org/architecture/) too? Render HTML from wasm ui components.

## Short Video Hype 

[![Wurbo Demo](https://i.ytimg.com/vi/x2ooLUTYuQk/oar2.jpg?sqp=-oaymwEaCN0CENAFSFXyq4qpAwwIARUAAIhCcAHAAQY=&rs=AOn4CLAyYw1c2XvGjZwgVg1RdG_mjE7s9Q)](https://www.youtube.com/shorts/x2ooLUTYuQk)

## Example

- Example Guest component is [`crates/vowels`](./crates/vowels/src/lib.rs)
- Example Host app is [`src/routes/+page.svelte`](./src/routes/+page.svelte)

## Demo

The example is demonstrated at [https://douganderson444.github.io/wurbo](https://douganderson444.github.io/wurbo/)

## Use Steps

1. In your WIT file:
- declare interface named `reactivity` with functions `render` and `activate`
- a separate interface (named anything you like, such as `imports`) with function `addeventlistener`

2. In the Rust: 
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
| 1 | declare interface `imports` | pass `imports` to macro | implement `imports` and pass stringified code to `rollup-plugin-wit-component` fn `load` via `importables` |
| 2 | declare interface `reactivity` | implement interface `reactvity` using the macro | load `reactivity` ES module using `rollup-plugin-wit-component` |
| 3 | Ready. | Ready. | call `mod.reactivity.render(args)` to get rendered HTML |
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

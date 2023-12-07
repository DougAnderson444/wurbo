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

## API 

This is ever changing experiment, so see the code for the latest API.

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

Compile the wasm component, see [README.md](./crates/vowels/README.md) for more details.

```bash
npm run dev

# or start the server and open the app in a new browser tab
npm run dev -- --open
```

## Building

To create a production version of your app:

```bash
npm run build
```

You can preview the production build with `npm run preview`.

> To deploy your app, you may need to install an [adapter](https://kit.svelte.dev/docs/adapters) for your target environment.

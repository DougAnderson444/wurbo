# Wurbo

[WIP] Like [Turbo](https://github.com/hotwired/turbo), but using [Wasm Components](https://github.com/WebAssembly/component-model). Kind of like [Elm](https://guide.elm-lang.org/architecture/) too? Render HTML from wasm ui components. Inject reactivity perhaps? Anything goes.

## Example

- Example Guest component is [`crates/hello`](./crates/hello/lib.rs)
- Example Host app is [`src/routes/+page.svelte`](./src/routes/+page.svelte)

## Demo

The example is demonstrated at [https://douganderson444.github.io/wurbo](https://douganderson444.github.io/wurbo/)

## Interactivity

1. Setup interactivity with HTMLElements by first activating them with `Interactive::activate(selector, event_type)`.

```rust
use render::{component, html, rsx, Render};

#[component]
pub fn Input<'a>(name: &'a str, id: &'a str) {
    // Type of event listener to listen for
    let ty = "keyup";

    // Add this CSS selector to the list of selectors that will add event listeners
    Interactive::activate(format!("#{id}"), ty);

    rsx! {
        <input id value={name} />
    }
}
```

2. Once your HTML has mounted to the DOM and the CSS selectors are available, your app calls `listen()` and the list of selectors are activated.

## Developing

Once you've created a project and installed dependencies with `npm install` (or `pnpm install` or `yarn`), start a development server:

Compile the wasm component, see [README.md](./crates/hello/README.md) for more details.

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

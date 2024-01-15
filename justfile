build-component:
  cargo component build --workspace

prev: build-component
  npm run build && npm run preview -- --open


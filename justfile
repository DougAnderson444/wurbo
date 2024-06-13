build-component:
  cargo component build --workspace

prev: build-component
  npm run build && npm run preview -- --open

npm-publish:
  npm install
  npm run package
  npm publish

cargo-publish:
  cargo publish -p wurbo

publish-all: npm-publish cargo-publish

#![allow(unused_braces)] // macro triggers this warning, disable it

use render::{
    // A macro to create components
    component,
    // A macro to render components in JSX fashion
    html,
    // A macro to compose components in JSX fashion
    rsx,
    // A trait for custom components
    Render,
};

// This can be any layout we want
#[component]
fn Page<'a, Children: Render>(title: &'a str, children: Children) {
    let world = "planet";
    rsx! {
      <>
        <h1 hello={world}>
            {title}
        </h1>
        <div>
            {children}
        </div>
      </>
    }
}

struct Component;

impl bindings::Example for Component {
    /// Say hello!
    fn render(name: String) -> String {
        // For IMPORTS: bindings         ::package::namespace::importname...
        // For EXPORTS: bindings::exports::package::namespace::exportname...
        // bindings::component::cargo_comp::imports::prnt("Hello, World!");

        html! {
          <Page title={"Home"}>
            {format!("Hello, {name}!")}
          </Page>
        }
    }
}

bindings::export!(Component);

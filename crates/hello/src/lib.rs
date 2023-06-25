#![allow(unused_braces)] // macro triggers this warning, disable it

mod button;
mod count;
mod input;

use button::Button;
use count::count_vowels;
use input::Input;

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
    let world = "planets";
    rsx! {
      <div class={"p-4"}>
        <h1 hello={world} class={"text-red-600 text-2xl font-bold"}>
            {title}
        </h1>
        <div>
            {children}
        </div>
      </div>
    }
}

struct Component;

impl bindings::Example for Component {
    /// Say hello!
    fn render(name: String) -> String {
        // For IMPORTS: bindings         ::package::namespace::importname...
        // For EXPORTS: bindings::exports::package::namespace::exportname...
        // bindings::component::cargo_comp::imports::prnt("Hello, World!");
        let count = count_vowels(&name);

        html! {
          <Page title={"Home"}>
            {format!("Hello, {name}!")}<br/>
            {format!("{name} has {count} vowels.")}<br/>
            <Input title={&name} />
            <Button title={"A title"}>
                {"Click Me!"}
            </Button>
          </Page>
        }
    }

    fn listen() {
        // call improts:: addeventlistener
        let selector = "button";
        let ty = "click";
        let val = "New Value";
        bindings::component::cargo_comp::imports::addeventlistener(selector, ty, val);
    }
}

bindings::export!(Component);

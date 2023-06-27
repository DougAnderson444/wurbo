//! # Wurbo
//! A Rust library for creating web components with JSX-like syntax.
//!
#![allow(unused_braces)] // macro triggers this warning, disable it

mod components;
mod input;
mod output;
mod utils;

use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::RwLock;

use components::page::Page;
use input::Input;
use output::Output;

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

///Maps the #elementId to the event type
type ListenerMap = HashMap<String, &'static str>;

// We cannot have &self in the Component struct,
// so we use static variables to store the state between functions
// See https://crates.io/crates/lazy_static
lazy_static::lazy_static! {
  // create Vec<bindings::component::cargo_comp::imports::ListenDetails>
  static ref LISTENERS_MAP: Mutex<ListenerMap> = Mutex::new(HashMap::new());
  // count
  static ref COUNT: RwLock<u32> = RwLock::new(0);
  // is_initialized
  static ref IS_INITIALIZED: RwLock<bool> = RwLock::new(false);
}

static OUTPUT_ID: OnceLock<String> = OnceLock::new();

pub struct Updater;

impl Updater {
    /// Insert the element id and event type into the LISTENERS_MAP
    pub fn update(elem_id: String, ty: &'static str) {
        let mut listeners = LISTENERS_MAP.lock().unwrap();
        listeners.insert(elem_id, ty);
    }
}

struct Component;

impl bindings::Example for Component {
    /// Say hello!
    fn render(name: String) -> String {
        let name = &name;

        // If you want to print in the console:
        // bindings::component::cargo_comp::imports::prnt(&format!("OUTPUT_ID? {OUTPUT_ID.get().unwrap()}"));

        if OUTPUT_ID.get().is_none() {
            #[allow(clippy::redundant_closure)]
            let id: &String = OUTPUT_ID.get_or_init(|| utils::rand_id());

            // Render and return all HTML
            html! {
              <Page title={"CAN'T BE EVIL"}>
                <Input name id={&utils::rand_id()} />
                <Output name id />
              </Page>
            }
        } else {
            // Render and return only the output section of HTML
            html! {
              <Output name id={OUTPUT_ID.get().unwrap()} />
            }
        }
    }

    fn listen() {
        // iterate through LISTENERS_MAP, add each using addeventlistener
        let listeners = LISTENERS_MAP.lock().unwrap();
        for (selector, ty) in listeners.iter() {
            let deets = bindings::component::cargo_comp::imports::ListenDetails {
                selector,
                ty,
                value: "TODO", // TODO: State vs Updates/Changes
            };

            bindings::component::cargo_comp::imports::addeventlistener(deets);
        }
    }
}

bindings::export!(Component);

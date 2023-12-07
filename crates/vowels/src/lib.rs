// the render crate needs braces to work
#![allow(unused_braces)]

cargo_component_bindings::generate!();

mod components;
mod input;
mod output;
mod utils;

use components::page::Page;
use input::Input;
use output::Output;

use crate::bindings::demo::vowels::imports;
use crate::bindings::Guest;

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
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::OnceLock;
use std::sync::RwLock;

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

/// The HTML element id of the output section so we can surgically render re-render it
static OUTPUT_ID: OnceLock<String> = OnceLock::new();

/// Insert the element id and event type into the LISTENERS_MAP
///
/// # Example
///
/// ```rust
/// let my_CSS_selector = "#some_selector";
/// Interactive::activate(format!("#{my_CSS_selector}"), "keyup");
/// ```
pub fn track(elem_id: String, ty: &'static str) {
    let mut listeners = LISTENERS_MAP.lock().unwrap();
    listeners.insert(elem_id, ty);
}

/// The WIT Component struct for implementing the Guest trait
struct Component;

impl Guest for Component {
    /// Say hello!
    fn render(name: String) -> String {
        let name = &name;

        // If you want to print in the console:
        // bindings::component::cargo_comp::imports::prnt(&format!("OUTPUT_ID? {OUTPUT_ID.get().unwrap()}"));

        // If OUTPUT_ID is not set, render the whole page since we are in the initial state
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
            // If OUTPUT_ID is set, render only the output section
            // This is so we keep our INPUT event listeners which were set above
            // Render and return only the output section of HTML
            html! {
              <Output name id={OUTPUT_ID.get().unwrap()} />
            }
        }
    }

    /// Activate the component listeners
    fn activate() {
        let listeners = LISTENERS_MAP.lock().unwrap();
        for (selector, ty) in listeners.iter() {
            let deets = imports::ListenDetails {
                selector: selector.to_string(),
                ty: ty.to_string(),
                value: "TODO".to_string(),
            };

            imports::addeventlistener(&deets);
        }
    }
}

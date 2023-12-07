// the render crate needs braces to work
#![allow(unused_braces)]

cargo_component_bindings::generate!();

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
use wurbo::generate_reactivity;

mod components;
mod input;
mod output;

use components::page::Page;
use input::Input;
use output::Output;

use crate::bindings::demo::vowels::imports;
use crate::bindings::exports::demo::vowels::reactivity::Guest as WurboGuest;
// use crate::bindings::Guest;

/// The WIT Component struct for implementing the Guest trait
struct Component;

// use the macro to generate and implement the WurboGuest implementations for your Components
generate_reactivity! { WurboGuest, Component, Page, Input, Output, imports }

// impl Guest for Component {
//  // other Guest functions can go here as required.
// }

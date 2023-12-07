// the render crate needs braces to work
#![allow(unused_braces)]

cargo_component_bindings::generate!();

mod components;
mod input;
mod output;
mod utils;

use wurbo::generate_reactivity;

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

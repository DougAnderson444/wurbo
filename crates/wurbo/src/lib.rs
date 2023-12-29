// the render crate needs braces to work
#![allow(unused_braces)]
#![doc = include_str!("../README.md")]

pub mod jinja;
pub mod rsx;
pub mod utils;

/// Prelude to ensure all users have the required deps
pub mod prelude {
    pub use crate::utils;
    pub use lazy_static::lazy_static;
    pub use minijinja::value::{StructObject, Value};
    pub use minijinja::Environment;
    // TODO: Remove render crate from repo
    pub use render::{
        // A macro to create components
        component,
        // A macro to render components in JSX fashion
        html,
        // A macro to compose components in JSX fashion
        rsx,
        // A trait for custom components
        Render,
    };
}

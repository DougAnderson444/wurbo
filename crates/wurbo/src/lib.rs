// the render crate needs braces to work
#![allow(unused_braces)]
#![feature(lazy_cell)]
#![doc = include_str!("../README.md")]

pub mod error;
pub mod jinja;
pub mod pest;
pub mod utils;

/// Prelude to ensure all users have the required deps
pub mod prelude {
    pub use crate::utils::{from_urlsafe, rand_id, variant_string, Base64JSON};
    pub use lazy_static::lazy_static;
    pub use minijinja::value::{Object, StructObject, Value};
    pub use minijinja::Environment;
}

// the render crate needs braces to work
#![allow(unused_braces)]

cargo_component_bindings::generate!();

use minijinja::value::StructObject;
use minijinja::Value;
use wurbo::jinja::Entry;
use wurbo::jinja::Index;
use wurbo::jinja::Rest;
use wurbo::jinja::Templates;
use wurbo::reactivity_bindgen;

// mod components;
// mod input;
// mod output;
mod count;
mod utils;

// use components::page::Page;
// use input::Input;
// use output::Output;

use crate::bindings::demo::vowels::imports;
use crate::bindings::demo::vowels::types::{self, Context as WitContext};
use crate::bindings::exports::demo::vowels::reactivity::Guest as WurboGuest;

/// The WIT Component struct for implementing the Guest trait
struct Component;

// impl Guest for Component {
//  // other Guest functions can go here as required.
// }

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    let templates = Templates::new(
        Index::new("page.html", include_str!("templates/page.html")),
        Rest::new(vec![
            Entry::new("input.html", include_str!("templates/input.html")),
            Entry::new("output.html", include_str!("templates/output.html")),
        ]),
    );
    templates
}

// Macro builds the Component struct and implements the Guest trait for us
reactivity_bindgen! {
    WurboGuest,
    Component,
    WitContext,
    imports
}

/// PageContext is the context with impl of StructObject
#[derive(Debug, Default, Clone)]
pub struct PageContext {
    page: Page,
    input: Input,
    output: Output,
}

impl StructObject for PageContext {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "page" => Some(Value::from_struct_object(self.page.clone())),
            "input" => Some(Value::from_struct_object(self.input.clone())),
            "output" => Some(Value::from_struct_object(self.output.clone())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["title", "id"])
    }
}

impl From<types::Content> for PageContext {
    fn from(context: types::Content) -> Self {
        PageContext {
            page: Page {
                title: context.page.title,
            },
            input: Input {
                placeholder: context.input.placeholder,
            },
            output: Output {
                name: context.output.name,
                id: context.output.id,
                template: context.output.template,
            },
        }
    }
}

/// Page is the wrapper for Input and Output
#[derive(Debug, Default, Clone)]
struct Page {
    title: String,
}

impl StructObject for Page {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "title" => Some(Value::from(self.title.clone())),
            "id" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["title", "id"])
    }
}

/// Input is the input form(s)
#[derive(Debug, Default, Clone)]
struct Input {
    placeholder: String,
}

impl StructObject for Input {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "placeholder" => Some(Value::from(self.placeholder.clone())),
            "id" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["placeholder", "id"])
    }
}

/// Output is the output area
#[derive(Debug, Default, Clone)]
struct Output {
    name: String,
    id: Option<String>,
    template: Option<String>,
}

impl Output {
    fn calculate(&self) -> Value {
        Value::from(count::count_vowels(&self.name))
    }
}

impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "name" => Some(Value::from(self.name.clone())),
            "count" => Some(Value::from(self.calculate())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["name", "count", "id"])
    }
}

impl From<types::Output> for Output {
    fn from(context: types::Output) -> Self {
        Output {
            name: context.name,
            id: context.id,
            template: context.template,
        }
    }
}

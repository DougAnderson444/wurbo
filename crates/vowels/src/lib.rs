// the render crate needs braces to work
#![allow(unused_braces)]

cargo_component_bindings::generate!();

use minijinja::value::StructObject;
use minijinja::Value;
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
use crate::bindings::demo::vowels::types;
use crate::bindings::exports::demo::vowels::reactivity;
// use crate::bindings::Guest;

/// The WIT Component struct for implementing the Guest trait
struct Component;

// use the macro to generate and implement the WurboGuest implementations for your Components
// generate_reactivity! { WurboGuest, Component, Page, Input, Output, imports }

// ($guest: ident, $component: ident, $templates:ident, $context:ident, $imports:ident) => {
// reactivity_bindgen! { WurboGuest, Component, templates, Context, imports }

// impl Guest for Component {
//  // other Guest functions can go here as required.
// }

fn get_templates() -> Vec<(&'static str, &'static str)> {
    let mut templates = Vec::new();
    templates.push(("page.html", include_str!("templates/page.html")));
    templates.push(("input.html", include_str!("templates/input.html")));
    templates.push(("output.html", include_str!("templates/output.html")));
    templates
}

reactivity_bindgen! {}

impl reactivity::Guest for Component {
    fn render(context: types::Context) -> Result<String, String> {
        let templates = get_templates();

        match context {
            types::Context::Content(c) => {
                let page_context = PageContext::from(c);
                Ok(wurbo::jinja::render(
                    "page.html",
                    &templates,
                    page_context,
                    Some(wurbo_tracker::track),
                )?)
            }
            types::Context::Output(o) => {
                let output = Output::from(o);
                // Build a PageContext with the given Output, as we need to pass an entire PageContext to the template
                // since the template uses "output.name", etc. this needs to be prepended. The
                // defaults are discarded in rendering since they don't apply to the output
                // template
                let pcontext = PageContext {
                    output: output.clone(),
                    ..Default::default()
                };
                Ok(wurbo::jinja::render(
                    "output.html",
                    &templates,
                    pcontext,
                    None,
                )?)
            }
        }
    }

    fn activate() {
        let listeners = LISTENERS_MAP.lock().unwrap();
        for (selector, (ty, outputid)) in listeners.iter() {
            let deets = imports::ListenDetails {
                selector: selector.to_string(),
                ty: ty.to_string(),
                outputid: outputid.to_string(),
            };

            imports::addeventlistener(&deets);
        }
    }
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
        }
    }
}

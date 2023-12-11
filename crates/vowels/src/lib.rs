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
    fn render(context: reactivity::Context) -> String {
        let templates = get_templates();

        imports::prnt(&format!("context: {:?}", context));
        if context.output.id.is_none() {
            let id: &String = OUTPUT_ID.get_or_init(|| utils::rand_id());

            // update the context with the new output id
            let mut context = context;
            context.output.id = Some(id.clone());

            wurbo::jinja::render(
                "page.html",
                &templates,
                PageContext::from(context),
                Some(wurbo_tracker::track),
            )
            .unwrap()
        } else {
            // just send the output context to "output.html"
            imports::prnt(&format!("output context: {:?}", context.output));
            wurbo::jinja::render("output.html", &templates, PageContext::from(context), None)
                .map_err(|e| {
                    imports::prnt(&format!("error: {:?}", e));
                    e
                })
                .unwrap()
        }
    }

    fn activate() {
        let listeners = LISTENERS_MAP.lock().unwrap();
        for (selector, ty) in listeners.iter() {
            let deets = imports::ListenDetails {
                selector: selector.to_string(),
                ty: ty.to_string(),
                outputid: OUTPUT_ID.get().unwrap().clone(),
            };

            imports::addeventlistener(&deets);
        }
    }
}

/// PageContext is the context with impl of StructObject
#[derive(Debug, Clone)]
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

impl From<reactivity::Context> for PageContext {
    fn from(context: reactivity::Context) -> Self {
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

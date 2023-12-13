cargo_component_bindings::generate!();

use std::ops::Deref;

use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

// mod components;
// mod input;
// mod output;
mod count;
mod utils;

// use components::page::Page;
// use input::Input;
// use output::Output;

use crate::bindings::demo::vowels::types::{self, Context};
use crate::bindings::demo::vowels::wurbo_in;
use crate::bindings::exports::demo::vowels::wurbo_out::Guest as WurboGuest;

/// The WIT Component struct for implementing the Guest trait
struct Component;

// impl Guest for Component {
//  // other Guest functions can go here as required.
// }

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    let templates = Templates::new(
        Index::new("page.html", include_str!("templates/page.html")),
        Entry::new("output.html", include_str!("templates/output.html")),
        Rest::new(vec![Entry::new(
            "input.html",
            include_str!("templates/input.html"),
        )]),
    );
    templates
}

// Macro builds the Component struct and implements the Guest trait for us
prelude_bindgen! {WurboGuest, Component, Context}

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
        Some(&["page", "input", "output"])
    }
}

impl From<&types::Context> for PageContext {
    fn from(context: &types::Context) -> Self {
        match context {
            types::Context::Content(c) => PageContext::from(c.clone()),
            types::Context::Phrase(p) => PageContext::from(Phrase::from(p)),
        }
    }
}

impl From<types::Content> for PageContext {
    fn from(context: types::Content) -> Self {
        PageContext {
            page: Page::from(context.page),
            input: Input::from(context.input),
            // We can use default for Output because the minijinja StructObject impl will
            // calculate the values from the above inouts for us
            output: Output::default(),
        }
    }
}

impl From<Phrase> for PageContext {
    fn from(context: Phrase) -> Self {
        // Safe to unwrap here because render on all page content will always be called first
        let state = { LAST_STATE.lock().unwrap().clone().unwrap() };
        PageContext {
            output: Output::from(context),
            ..state
        }
    }
}

/// Page is the wrapper for Input and Output
#[derive(Debug, Clone)]
struct Page(types::Page);

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

impl From<types::Page> for Page {
    fn from(page: types::Page) -> Self {
        Page(page)
    }
}

impl Deref for Page {
    type Target = types::Page;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Input is the input form(s)
#[derive(Debug, Clone)]
struct Input(types::Input);

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

impl From<types::Input> for Input {
    fn from(context: types::Input) -> Self {
        Input(context)
    }
}

impl Deref for Input {
    type Target = types::Input;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Output is the output area
#[derive(Debug, Default, Clone)]
struct Output {
    value: String,
    id: Option<String>,
}

impl Output {
    fn calculate(&self) -> Value {
        Value::from(count::count_vowels(&self.value))
    }
}

/// This is where the magic happens. Calling `render` executes `get_field` which will take the
/// value as input and generate a new `count` which is then displayed in the `template`.
impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(self.value.clone())),
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
            value: context.value,
            id: context.id,
        }
    }
}

impl From<Phrase> for Output {
    fn from(context: Phrase) -> Self {
        Output {
            value: context.value.clone(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Phrase(Option<types::Outrecord>);

impl StructObject for Phrase {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "phrase" => Some(Value::from(self.value.clone())),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["phrase"])
    }
}

impl From<&types::Outrecord> for Phrase {
    fn from(context: &types::Outrecord) -> Self {
        Phrase(Some(context.clone()))
    }
}

impl From<Option<types::Outrecord>> for Phrase {
    fn from(context: Option<types::Outrecord>) -> Self {
        Phrase(context)
    }
}

impl Deref for Phrase {
    type Target = types::Outrecord;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

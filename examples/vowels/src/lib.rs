use std::ops::Deref;

use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

// mod components;
// mod input;
// mod output;
mod bindings;
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
bindings::export!(Component with_types_in bindings);

// impl Guest for Component {
//  // other Guest functions can go here as required.
// }

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    let templates = Templates::new(
        Index::new(
            "page.html".to_owned(),
            include_str!("templates/page.html").to_owned(),
        ),
        Entry::new(
            "output.html".to_owned(),
            include_str!("templates/output.html").to_owned(),
        ),
        Rest::new(vec![Entry::new(
            "input.html".to_owned(),
            include_str!("templates/input.html").to_owned(),
        )]),
    );
    templates
}

// Macro builds the Component struct and implements the Guest trait for us
prelude_bindgen! {WurboGuest, Component, PageContext, Context, LAST_STATE}

/// PageContext is the context with impl of StructObject
#[derive(Debug, Clone)]
pub struct PageContext {
    page: Page,
    input: Input,
    output: Output,
    target: Option<String>,
}

impl Object for PageContext {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "id" => Some(Value::from(rand_id())),
            "page" => Some(Value::from_object(self.page.clone())),
            "input" => Some(Value::from_object(self.input.clone())),
            "output" => Some(Value::from_object(self.output.clone())),
            _ => None,
        }
    }
}

impl From<&types::Context> for PageContext {
    fn from(context: &types::Context) -> Self {
        match context {
            types::Context::AllContent(c) => PageContext::from(c.clone()),
            types::Context::Phrase(p) => PageContext::from(Phrase::from(p.clone())),
        }
    }
}

impl From<types::Content> for PageContext {
    fn from(context: types::Content) -> Self {
        PageContext {
            page: Page::from(context.page),
            input: Input::from(context.input),
            output: Output::from(context.output),
            target: None, // use default
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

impl Object for Page {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "title" => Some(Value::from(self.title.clone())),
            "id" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
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

impl Object for Input {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "placeholder" => Some(Value::from(self.placeholder.clone())),
            "id" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
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
impl Object for Output {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "value" => Some(Value::from(self.value.clone())),
            "count" => Some(Value::from(self.calculate())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
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

impl From<Option<types::Output>> for Output {
    fn from(context: Option<types::Output>) -> Self {
        match context {
            Some(context) => Output::from(context),
            None => Default::default(),
        }
    }
}

impl From<Phrase> for Output {
    fn from(context: Phrase) -> Self {
        Output {
            value: context.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Phrase(Option<String>);

impl Object for Phrase {
    fn get_value(self: &std::sync::Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "phrase" => Some(Value::from(self.as_ref().deref().clone())),
            _ => None,
        }
    }
}

impl From<String> for Phrase {
    fn from(context: String) -> Self {
        Phrase(Some(context.clone()))
    }
}

impl From<Option<String>> for Phrase {
    fn from(context: Option<String>) -> Self {
        Phrase(context)
    }
}

impl Deref for Phrase {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref().unwrap()
    }
}

mod bindings;
mod input;
mod output;
mod page;

use input::Input;
use output::Output;
use page::Page;

use std::{ops::Deref, sync::Arc};

use bindings::demo::forms::{
    types::{self, Context},
    wurbo_in,
};
// We will likely have other guests, so let's alias this one to WurboGuest
use bindings::exports::demo::forms::wurbo_out::Guest as WurboGuest;
use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

/// The struct for the bound component that implements the Guest traits
struct Component;
bindings::export!(Component with_types_in bindings);

const OUTPUT_HTML: &str = "output.html";
const INDEX_HTML: &str = "index.html";
const INPUT_HTML: &str = "input.html";

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    let templates = Templates::new(
        Index::new(INDEX_HTML, include_str!("templates/index.html")),
        Entry::new(OUTPUT_HTML, include_str!("templates/output.html")),
        Rest::new(vec![Entry::new(
            INPUT_HTML,
            include_str!("templates/input.html"),
        )]),
    );
    templates
}

// Macro builds the Component struct and implements the Guest trait for us, saving copy-and-paste
prelude_bindgen! {WurboGuest, Component, PageContext, Context, LAST_STATE}

/// PageContext is a struct of other structs that implement [`minijinja::value::Object`],
/// which is why it is not a Newtype wrapper like the others are.
#[derive(Debug, Clone)]
pub struct PageContext {
    page: Page,
    input: Input,
    pub(crate) output: Output,
    target: Option<String>,
}

impl Object for PageContext {
    fn get_value(self: &Arc<Self>, field: &Value) -> Option<Value> {
        match field.as_str()? {
            "page" => Some(Value::from_object(self.page.clone())),
            "input" => Some(Value::from_object(self.input.clone())),
            "output" => Some(Value::from_object(self.output.clone())),
            _ => None,
        }
    }
}

/// We received Context from the WIT ABO and need to convert it to PageContext
impl From<&types::Context> for PageContext {
    fn from(context: &types::Context) -> Self {
        // Output is not a type of context, because it is calculated from the other values
        match context {
            types::Context::AllContent(c) => PageContext::from(c.clone()),
            types::Context::Username(u) => PageContext::from(output::Username::from(u.to_owned())),
            types::Context::Password(p) => PageContext::from(output::Password::from(p.to_owned())),
        }
    }
}

/// We have all the content, convert it to PageContext
impl From<types::Content> for PageContext {
    fn from(context: types::Content) -> Self {
        PageContext {
            page: Page::from(context.page),
            input: Input::from(context.input),
            // We can use default for Output because the minijinja StructObject impl will
            // calculate the values from the above inouts for us
            output: Output::default(),
            // No override for target, use the default
            target: None,
        }
    }
}

impl From<output::Username> for PageContext {
    fn from(username: output::Username) -> Self {
        // Safe to unwrap here because render on all page content will always be called first
        let mut state = { LAST_STATE.lock().unwrap().clone().unwrap() };
        state.output.username = username;
        state
    }
}

impl From<output::Password> for PageContext {
    fn from(password: output::Password) -> Self {
        // Safe to unwrap here because render on all page content will always be called first
        let mut state = { LAST_STATE.lock().unwrap().clone().unwrap() };
        state.output.password = password;
        state
    }
}

#[cfg(test)]
mod test_forms_ui {
    use super::*;

    #[test]
    fn test_get_context_fields() {}
}

cargo_component_bindings::generate!();

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

/// We need to provide the templates for the macro to pull in
fn get_templates() -> Templates {
    let templates = Templates::new(
        Index::new("page.html", include_str!("templates/page.html")),
        Entry::new("output.html", include_str!("templates/output.html")),
        Rest::new(vec![
            Entry::new("input.html", include_str!("templates/input.html")),
            Entry::new("username.html", include_str!("templates/username.html")),
        ]),
    );
    templates
}

// Macro builds the Component struct and implements the Guest trait for us, saving copy-and-paste
prelude_bindgen! {WurboGuest, Component, Context}

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

impl From<&types::Context> for PageContext {
    fn from(context: &types::Context) -> Self {
        match context {
            types::Context::Content(c) => PageContext::from(c.clone()),
            types::Context::Username(u) => PageContext::from(Username::from(u)),
            types::Context::Password(p) => PageContext::from(Password::from(p)),
            types::Context::Output(o) => PageContext::from(o.clone()),
        }
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
            output: Output::from(context.output),
        }
    }
}

impl From<types::Output> for PageContext {
    fn from(context: types::Output) -> Self {
        let state = { LAST_STATE.lock().unwrap().clone() };
        PageContext {
            output: Output::from(context),
            ..state
        }
    }
}

impl From<Output> for PageContext {
    fn from(output: Output) -> Self {
        let state = { LAST_STATE.lock().unwrap().clone() };
        PageContext { output, ..state }
    }
}

impl From<Username> for PageContext {
    fn from(username: Username) -> Self {
        let state = { LAST_STATE.lock().unwrap().clone() };
        let pc = PageContext {
            output: Output {
                username,
                ..state.output
            },
            ..state
        };
        pc
    }
}

impl From<Password> for PageContext {
    fn from(password: Password) -> Self {
        let state = { LAST_STATE.lock().unwrap().clone() };
        let pc = PageContext {
            output: Output {
                password,
                ..state.output
            },
            ..state
        };
        pc
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
            "username_input" => Some(Value::from(utils::rand_id())),
            "password_input" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
    }
    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["placeholder", "username_input", "password_input"])
    }
}

/// Output is the output area
#[derive(Debug, Default, Clone)]
struct Output {
    id: Option<String>,
    username: Username,
    password: Password,
}

impl Output {
    fn calculate(&self) -> Value {
        // const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];
        //
        // pub fn count_vowels(s: &str) -> usize {
        //     s.chars().filter(|c| VOWELS.contains(c)).count()
        // }

        Value::from(*&self.concat().len())
    }

    fn concat(&self) -> String {
        format!("{}{}", self.username.value, self.password.value)
    }
}

impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "username" => Some(Value::from_struct_object(self.username.clone())),
            "password" => Some(Value::from_struct_object(self.password.clone())),
            "value" => Some(Value::from(self.concat())),
            // self.username.value
            "count" => Some(Value::from(self.calculate())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value", "count", "id", "username", "password"])
    }
}

impl From<types::Output> for Output {
    fn from(context: types::Output) -> Self {
        Output {
            id: context.id,
            username: Username::from(context.username),
            password: Password::from(context.password),
        }
    }
}

impl From<Option<types::Output>> for Output {
    fn from(context: Option<types::Output>) -> Self {
        match context {
            Some(c) => Output::from(c),
            None => Output::default(),
        }
    }
}

/// Username is the username input.
/// We need a separate template for Username so we can direct the prop there.
/// If we have a separate template, we need a separate struct?
#[derive(Debug, Default, Clone)]
struct Username {
    // the value that is passed to the template as the prop
    value: String,
}

impl StructObject for Username {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(self.value.clone())),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value", "id"])
    }
}

impl From<&types::Outrecord> for Username {
    fn from(context: &types::Outrecord) -> Self {
        Username {
            value: context.value.to_string(),
        }
    }
}

impl From<Option<types::Outrecord>> for Username {
    fn from(context: Option<types::Outrecord>) -> Self {
        match context {
            Some(c) => Username::from(&c),
            None => Username::default(),
        }
    }
}

/// Password is the password field in the form
#[derive(Debug, Default, Clone)]
struct Password {
    // the value that is passed to the template as the prop
    value: String,
}

impl StructObject for Password {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(self.value.clone())),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value", "id"])
    }
}

impl From<&types::Outrecord> for Password {
    fn from(context: &types::Outrecord) -> Self {
        Password {
            value: context.value.to_string(),
        }
    }
}

impl From<Option<types::Outrecord>> for Password {
    fn from(context: Option<types::Outrecord>) -> Self {
        match context {
            Some(c) => Password::from(&c),
            None => Password::default(),
        }
    }
}

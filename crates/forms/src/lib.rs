cargo_component_bindings::generate!();

use std::ops::Deref;

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
prelude_bindgen! {WurboGuest, Component, PageContext, Context}

/// PageContext is a struct of other structs that implement [StructObject],
/// which is why it is not a Newtype wrapper like the others are.
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

/// We received Context from the WIT ABO and need to convert it to PageContext
impl From<&types::Context> for PageContext {
    fn from(context: &types::Context) -> Self {
        // Output is not a type of context, because it is calculated from the other values
        match context {
            types::Context::Content(c) => PageContext::from(c.clone()),
            types::Context::Username(u) => PageContext::from(Username::from(u)),
            types::Context::Password(p) => PageContext::from(Password::from(p)),
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
        }
    }
}

impl From<Username> for PageContext {
    fn from(username: Username) -> Self {
        // Safe to unwrap here because render on all page content will always be called first
        let mut state = { LAST_STATE.lock().unwrap().clone().unwrap() };
        state.output.username = username;
        state
    }
}

impl From<Password> for PageContext {
    fn from(password: Password) -> Self {
        // Safe to unwrap here because render on all page content will always be called first
        let mut state = { LAST_STATE.lock().unwrap().clone().unwrap() };
        state.output.password = password;
        state
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
    fn from(context: types::Page) -> Self {
        Page(context)
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

/// Output handles the storage of the values and the calculation of the length of the concatenated
#[derive(Debug, Default, Clone)]
struct Output {
    id: Option<String>,
    username: Username,
    password: Password,
}

impl Output {
    /// Calculate the length of the username and password concatenated
    fn calculate(&self) -> Value {
        Value::from(*&self.concat().len())
    }

    /// Concatenate the username and password
    fn concat(&self) -> String {
        format!(
            "{}{}",
            self.username
                .as_ref()
                .map(|v| v.value.clone())
                .unwrap_or_default(),
            self.password
                .as_ref()
                .map(|v| v.value.clone())
                .unwrap_or_default()
        )
    }
}

/// Impleent StructObject for Output so we can use minijina to automagically calculate the length
/// of the username and password concatenated
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

/// Username captures is the username input value.
#[derive(Debug, Default, Clone)]
struct Username(Option<types::Outrecord>);

impl StructObject for Username {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref().map(|v| v.value.clone()).unwrap_or_default(),
            )),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value"])
    }
}

impl From<&types::Outrecord> for Username {
    fn from(context: &types::Outrecord) -> Self {
        Username(Some(context.clone()))
    }
}

impl From<Option<types::Outrecord>> for Username {
    fn from(context: Option<types::Outrecord>) -> Self {
        Username(context)
    }
}

impl Deref for Username {
    type Target = Option<types::Outrecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [Password] is the password field in the form
/// We wrap it as a newtype so that we can impl [StructObject] for it
/// We impl [Deref] so we can access the inner of the Rust smart pointer
#[derive(Debug, Default, Clone)]
struct Password(Option<types::Outrecord>);

impl StructObject for Password {
    // If you add fields to the Outrecord, you'd add them also here below:
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref().map(|v| v.value.clone()).unwrap_or_default(),
            )),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value"])
    }
}

impl From<&types::Outrecord> for Password {
    fn from(context: &types::Outrecord) -> Self {
        Password(Some(context.clone()))
    }
}

impl From<Option<types::Outrecord>> for Password {
    fn from(context: Option<types::Outrecord>) -> Self {
        Password(context)
    }
}

impl Deref for Password {
    type Target = Option<types::Outrecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

cargo_component_bindings::generate!();

use bindings::demo::forms::{
    types::{self, Context as WitContext},
    wurbo_in,
};
use bindings::exports::demo::forms::wurbo_out::Guest as WurboGuest;
use wurbo::jinja::{Entry, Index, Rest, Templates};
use wurbo::prelude_bindgen;

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
prelude_bindgen! {}

impl WurboGuest for Component {
    fn render(context: WitContext) -> Result<String, String> {
        // TODO: pass in the templates to the macro.
        let templates = get_templates();

        println!("rendering ctx: {:?}", context);

        if let WitContext::Content(c) = context {
            let page_context = PageContext::from(c);
            // update cache
            let mut last_state = LAST_STATE.lock().unwrap();
            *last_state = page_context.clone();

            Ok(wurbo::jinja::render(
                templates.entry.name,
                templates,
                page_context,
                Some(wurbo_tracker::track),
            )?)
        } else {
            //                let output = Output::from(context);
            // Build a PageContext with the given Output, as we need to pass an entire PageContext to the template
            // since the template uses "output.name", etc. this needs to be prepended. The
            // defaults are discarded in rendering since they don't apply to the output
            // template
            // merge context updates with current state
            let page_context = PageContext::from(&context.clone());
            // update cache to pcontext
            let mut last_state = LAST_STATE.lock().unwrap();
            *last_state = page_context.clone();

            Ok(wurbo::jinja::render(
                // The chosen template to update
                &templates.output.name.to_string(),
                // Pass all the template for reference
                templates,
                // Pass the whole Page context, as that is what the templates expect
                page_context,
                // We're not registering any listeners here, so we can pass None
                None,
            )?)
        }
        // WitContext::Username(u) => {
        //     let username = Username::from(u);
        //     // merge context updates with current state
        //     let page_context = PageContext::from(username.clone());
        //     // update cache to pcontext
        //     let mut last_state = LAST_STATE.lock().unwrap();
        //     *last_state = page_context.clone();
        //
        //     Ok(wurbo::jinja::render(
        //         // The chosen template to update
        //         &templates.output.name.to_string(),
        //         // Pass all the template for reference
        //         templates,
        //         // Pass the whole Page context, as that is what the templates expect
        //         page_context,
        //         // We're not registering any listeners here, so we can pass None
        //         None,
        //     )?)
        // }
        // }
    }

    fn activate() {
        let listeners = LISTENERS_MAP.lock().unwrap();
        for (selector, (ty, template)) in listeners.iter() {
            let deets = wurbo_in::ListenDetails {
                selector: selector.to_string(),
                ty: ty.to_string(),
                template: template.to_string(),
            };

            wurbo_in::addeventlistener(&deets);
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

impl From<&types::Context> for PageContext {
    fn from(context: &types::Context) -> Self {
        match context {
            types::Context::Content(c) => PageContext::from(c.clone()),
            types::Context::Username(u) => PageContext::from(u.clone()),
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
        println!("username pc: {:?}", pc);
        pc
    }
}

impl From<types::Outrecord> for PageContext {
    fn from(outrecord: types::Outrecord) -> Self {
        let state = { LAST_STATE.lock().unwrap().clone() };
        let pc = PageContext {
            output: Output {
                username: Username {
                    value: outrecord.value,
                    id: outrecord.id,
                    template: outrecord.template,
                },
                ..state.output
            },
            ..state
        };
        println!("outrecord pc: {:?}", pc);
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
    // the value that is passed to the template as the prop
    value: String,
    // the id of the element
    id: Option<String>,
    // the teplate string to use as "entry" during `render`
    // discarded after `render` is called
    template: Option<String>,
    username: Username,
    // password: Password,
}

impl Output {
    fn calculate(&self) -> Value {
        const VOWELS: &[char] = &['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'];

        pub fn count_vowels(s: &str) -> usize {
            s.chars().filter(|c| VOWELS.contains(c)).count()
        }

        Value::from(count_vowels(&self.username.value))
    }
}

impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "username" => Some(Value::from_struct_object(self.username.clone())),
            "value" => Some(Value::from(self.username.value.clone())),
            // self.username.value
            "count" => Some(Value::from(self.calculate())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value", "count", "id"])
    }
}

impl From<types::Output> for Output {
    fn from(context: types::Output) -> Self {
        Output {
            value: context.value,
            id: context.id,
            template: context.template,
            username: Username {
                value: context.username.value,
                id: context.username.id,
                template: context.username.template,
            },
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
    // the id of the element
    id: Option<String>,
    // the teplate string to use as "entry" during `render`
    // discarded after `render` is called
    template: Option<String>,
}

impl StructObject for Username {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(self.value.clone())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(utils::rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value", "id"])
    }
}

impl From<types::Outrecord> for Username {
    fn from(context: types::Outrecord) -> Self {
        Username {
            value: context.value,
            id: context.id,
            template: context.template,
        }
    }
}

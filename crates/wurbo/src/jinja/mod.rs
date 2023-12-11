//! Combines data + template to render the HTML
//!
//! Instead of passing `render` crate `components` to the macro, you pass:
//! 1. Page, Input and Output as minijinja templates
//! 2. The state data (as JSON?)
//!
//! The output becomes:
//! 1. The rendered HTML
//! 2. The latest state data as context
//!
//! The minijinja module will convert strings like `on:keyup` to `onkeyup` for you.
//!
//! The user can then make use of minijinja's:
//! - imports: to compose components `{% from "my_template.html" import my_macro, my_variable %}`
//! - macros: to write reusable template functions `{% macro input(name, id) %}...{% endmacro %}`
//! - inheritance: to extend templates (templates within templates) `{% extends "base.html" %}`
//! - include: to include templates (templates within templates) `{% include "header.html" %}`
use std::ops::Deref;

use crate::utils;
mod count;
mod error;

use minijinja::value::{StructObject, Value};
use minijinja::Environment;

/// Represents the context for the page
#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct PageContext {
    page: Page,
    input: Input,
    output: Output,
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
struct Output {
    name: String,
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
            "id" => Some(Value::from(utils::rand_id())),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["name", "count", "id"])
    }
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

/// This struct and it's impls replaces the templates Vec with a type that identifies an Entry
/// tuple,
pub struct Templates {
    pub entry: Entry,
    pub rest: Vec<Entry>,
}

impl Templates {
    pub fn new(entry: Index, rest: Rest) -> Self {
        Self {
            entry: entry.0,
            rest: rest.0,
        }
    }
}

pub struct Index(Entry);

impl Index {
    pub fn new(name: &'static str, template: &'static str) -> Self {
        Self(Entry::new(name, template))
    }
}

impl Deref for Index {
    type Target = Entry;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Rest(Vec<Entry>);

impl Rest {
    pub fn new(vals: Vec<Entry>) -> Self {
        Self(vals)
    }

    pub fn push(&mut self, entry: Entry) {
        self.0.push(entry);
    }
}

impl Deref for Rest {
    type Target = Vec<Entry>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct Entry {
    pub name: &'static str,
    pub template: &'static str,
}

impl Entry {
    pub fn new(name: &'static str, template: &'static str) -> Self {
        Self { name, template }
    }
}

/// Impl IntoIterator for Templates
impl IntoIterator for Templates {
    type Item = (&'static str, &'static str);
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut v = Vec::new();
        v.push((self.entry.name, self.entry.template));
        for e in self.rest {
            v.push((e.name, e.template));
        }
        v.into_iter()
    }
}

/// Render the given entry filename with the given templates using the context
/// Trackers will register any event listeners as per the templates' `on` attributes
pub fn render(
    entry: &str,
    // The chosen template to update
    templates: Templates,
    // as long as ctx_struct implements StructObject, we can use it
    ctx_struct: impl StructObject + 'static,
    tracker: Option<fn(String, String, String, String) -> String>,
) -> Result<String, error::RenderError> {
    let mut env = Environment::new();

    for (name, template) in templates.into_iter() {
        env.add_template(name, template)
            .expect("template should be added");
    }

    if let Some(tracker) = tracker {
        env.add_filter("on", tracker);
    }

    let ctx = Value::from_struct_object(ctx_struct);

    let tmpl = env.get_template(entry)?;

    let rendered = match tmpl.render(&ctx) {
        Ok(rendered) => rendered,
        Err(e) => {
            println!("Could not render template: {:#}", e);
            // render causes as well
            let mut err = &e as &dyn std::error::Error;
            while let Some(next_err) = err.source() {
                println!();
                println!("caused by: {:#}", next_err);
                err = next_err;
            }
            return Err(error::RenderError::from(e));
        }
    };
    Ok(rendered)
}

#[macro_export]
macro_rules! reactivity_bindgen {
    () => {
        use $crate::prelude::*;

        use std::collections::HashMap;
        use std::sync::Mutex;
        use std::sync::OnceLock;
        use std::sync::RwLock;

        ///Maps the #elementId to the event type
        type ListenerMap = HashMap<String, (String, String, String)>;

        // We cannot have &self in the Component struct,
        // so we use static variables to store the state between functions
        // See https://crates.io/crates/lazy_static
        lazy_static! {
          // create Vec<bindings::component::cargo_comp::imports::ListenDetails>
          static ref LISTENERS_MAP: Mutex<ListenerMap> = Mutex::new(HashMap::new());
          // is_initialized
          static ref IS_INITIALIZED: RwLock<bool> = RwLock::new(false);
        }

        /// The HTML element id of the output section so we can surgically render re-render it
        static OUTPUT_ID: OnceLock<String> = OnceLock::new();

        // unique namespace to clairfy and avoid collisions with other Guest code
        mod wurbo_tracker {
            /// Insert the source element id, event type, and target output id into the LISTENERS_MAP
            ///
            /// # Example
            ///
            /// ```rust
            /// let my_CSS_selector = "#some_selector";
            /// wurbo_tracker::track(format!("#{my_CSS_selector}"), "keyup", "my_output_id");
            /// ```
            pub fn track(
                elem_id: String,
                ty: String,
                outputid: String,
                template: String,
            ) -> String {
                let mut listeners = super::LISTENERS_MAP.lock().unwrap();
                listeners.insert(format!("#{elem_id}"), (ty, outputid, template));
                elem_id
            }
        }

        // impl $guest for $component {
        //     /// Say hello!
        //     fn render(context: $context) -> String {
        //         let name = &name;
        //
        //         if OUTPUT_ID.get().is_none() {
        //             #[allow(clippy::redundant_closure)]
        //             let id: &String = OUTPUT_ID.get_or_init(|| utils::rand_id());
        //
        //             // Call Render on the "page.html" template and return all HTML
        //             let rendered = $crate::render("page.html", &$templates, context.into())
        //                 .map_err(|e| format!("Error rendering page.html: {:?}", e))
        //                 .unwrap();
        //             rendered
        //         } else {
        //             // If OUTPUT_ID is set, render only the output section
        //             // This is so we keep our INPUT event listeners which were set above
        //             // Render and return only the output section of HTML
        //             let rendered = $crate::render("output.html", &$templates, context.into())
        //                 .map_err(|e| {
        //                     format!(
        //                         "Error rendering output.html: {:?} for context: {:?}",
        //                         e, context
        //                     )
        //                 })
        //                 .unwrap();
        //             rendered
        //         }
        //     }
        //
        //     /// Activate the component listeners
        //     fn activate() {
        //         let listeners = LISTENERS_MAP.lock().unwrap();
        //         for (selector, ty) in listeners.iter() {
        //             let deets = $imports::ListenDetails {
        //                 selector: selector.to_string(),
        //                 ty: ty.to_string(),
        //                 value: "TODO".to_string(),
        //             };
        //
        //             $imports::addeventlistener(&deets);
        //         }
        //     }
        // }
    };
}

#[cfg(test)]
mod jinja_unit_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn smoke() -> Result<(), Box<dyn std::error::Error>> {
        let ctx_struct = PageContext {
            page: Page {
                title: "Let's count vowels using templates for Inputs and Outputs!".to_string(),
            },
            input: Input {
                placeholder: "Input the word with vowels it's here".to_string(),
            },
            output: Output {
                name: "vowels".to_string(),
            },
        };

        let cwd = std::env::current_dir()?;
        let path = PathBuf::from(cwd).join("./src/jinja/fixtures");
        std::fs::write(path.join("data.json"), serde_json::to_string(&ctx_struct)?)?;

        let mut tmpls = Vec::new();
        tmpls.push(("page.html", include_str!("fixtures/page.html")));
        tmpls.push(("input.html", include_str!("fixtures/input.html")));
        tmpls.push(("output.html", include_str!("fixtures/output.html")));

        // mock tracker
        fn tracker(_elem_id: String, _ty: String) {}

        let rendered_page =
            render("page.html", &tmpls, ctx_struct.clone(), tracker).expect("test should pass");

        eprintln!("rendered_page:\n{}", rendered_page);

        // write rendered_page to file
        std::fs::write(path.join("rendered_page.html"), rendered_page)?;

        eprintln!("\nNow, let's render just the Output \n");

        let rendered_output =
            render("output.html", &tmpls, ctx_struct, tracker).expect("test should pass");

        eprintln!("OUTPUT ONLY:\n{}", rendered_output);

        Ok(())
    }
}

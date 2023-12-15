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

mod error;

use minijinja::value::StructObject;
use minijinja::Environment;
use minijinja::Value;

/// This struct and it's impls replaces the templates Vec with a type that identifies an Entry
/// tuple,
pub struct Templates {
    pub entry: Entry,
    pub output: Entry,
    pub rest: Vec<Entry>,
}

impl Templates {
    pub fn new(entry: Index, output: Entry, rest: Rest) -> Self {
        Self {
            entry: entry.0,
            output,
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
        v.push((self.output.name, self.output.template));
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
    tracker: fn(String, String) -> String,
) -> Result<String, error::RenderError> {
    let mut env = Environment::new();

    for (name, template) in templates.into_iter() {
        env.add_template(name, template)
            .expect("template should be added");
    }

    env.add_filter("on", tracker);

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

/// Implements the wurbo_out::Guest trait for the given component.
///
/// You must have a `PageContext` struct that implements StructObject in the same file where you use
/// this macro.
///
/// This macro also creates and makes available a `LAST_STATE` static variable
/// that stores the last [PageContext] that was rendered.
#[macro_export]
macro_rules! prelude_bindgen {
    ( 
        $guest: ident,
        $component: ident,
        $pagecontext: ident,
        $context:ident
) => {
        use $crate::prelude::*;

        use std::collections::HashMap;
        use std::sync::Mutex;

        ///Maps the #elementId to the event type
        type ListenerMap = HashMap<String, String>;

        // We cannot have &self in the Component struct,
        // so we use static variables to store the state between functions
        // See https://crates.io/crates/lazy_static
        lazy_static! {
        // create Vec<bindings::component::cargo_comp::imports::ListenDetails>
        static ref LISTENERS_MAP: Mutex<ListenerMap> = Mutex::new(HashMap::new());
        // cache the last state of $pagecontext as Mutex
        static ref LAST_STATE: Mutex<Option<$pagecontext>> = Mutex::new(None);
        }

        /// unique namespace to clairfy and avoid collisions with other Guest code
        mod wurbo_tracker {
            /// Insert the source element id, event type, and target output id into the LISTENERS_MAP
            ///
            /// # Example
            ///
            /// ```rust
            /// let my_CSS_selector = "#some_selector";
            /// wurbo_tracker::track(format!("#{my_CSS_selector}"), "keyup");
            /// ```
            pub fn track(elem_id: String, ty: String) -> String {
                let mut listeners = super::LISTENERS_MAP.lock().unwrap();
                // This is how you specify a selector that is the id_child of the parent with
                // id_parent:
                // let selector = format!("#{} #{}", id_parent, id_child);
                listeners.insert(format!("#{elem_id}"), ty);
                elem_id
            }
        }

        impl $guest for $component {
            fn render(context: $context) -> Result<String, String> {
                // TODO: pass in the templates to the macro.
                let templates = get_templates();
                let page_context = $pagecontext::from(&context);
                // update cache
                let mut last_state = LAST_STATE.lock().unwrap();
                *last_state = Some(page_context.clone());

                // check whether context has page and input fields in it or not

                let entry = match context {
                    $context::AllContent(_) => templates.entry.name,
                    _ => templates.output.name,
                };

                Ok(wurbo::jinja::render(
                    entry,
                    templates,
                    page_context,
                    wurbo_tracker::track,
                )?)
            }

            fn activate() {
                let listeners = LISTENERS_MAP.lock().unwrap();
                for (selector, ty) in listeners.iter() {
                    let deets = wurbo_in::ListenDetails {
                        selector: selector.to_string(),
                        ty: ty.to_string(),
                    };

                    wurbo_in::addeventlistener(&deets);
                }
            }
        }
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

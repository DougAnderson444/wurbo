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

pub mod error;

pub use minijinja::context;
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

/// Indexes a string.
/// # Example
/// mystring -> mystring[42]
fn indexify(given: String, index: usize) -> String {
    format!("{given}[{index}]")
}

/// Render the given entry filename with the given templates using the context
/// Trackers will register any event listeners as per the templates' `on` attributes
pub fn render(
    entry: &str,
    // The chosen template to update
    templates: Templates,
    // as long as ctx_struct implements StructObject, we can use it
    ctx: Value,
    tracker: fn(String, String) -> String,
) -> Result<String, error::RenderError> {
    let mut env = Environment::new();

    for (name, template) in templates.into_iter() {
        env.add_template(name, template)
            .expect("template should be added");
    }

    env.add_filter("on", tracker);
    env.add_filter("append", indexify);

    let tmpl = env.get_template(entry)?;

    let rendered = tmpl.render(&ctx).map_err(|e| {
        println!("Could not render template: {:#}", e);
        // render causes as well
        let mut err = &e as &dyn std::error::Error;
        while let Some(next_err) = err.source() {
            println!("caused by: {:#}", next_err);
            err = next_err;
        }
        error::RenderError::from(e)
    })?;
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
        $context:ident,
        $state:ident
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
        static ref $state: Mutex<Option<$pagecontext>> = Mutex::new(None);
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

                // print warning if elem_id string is empty for listener type 'ty'
                if elem_id.is_empty() {
                    eprintln!("WARNING: elem_id is empty for listener type '{}'", ty);
                }

                // This is how you specify a selector that is the id_child of the parent with
                // id_parent:
                // let selector = format!("#{} #{}", id_parent, id_child);
                listeners.insert(format!("#{elem_id}"), ty);
                elem_id
            }
        }

        impl $guest for $component {
            fn render(context: $context, target: String) -> Result<String, String> {
                // TODO: pass in the templates to the macro.
                let templates = get_templates();
                let page_context = $pagecontext::from(&context);
                // update cache
                let mut last_state = $state.lock().unwrap();
                *last_state = Some(page_context.clone());

                let ctx = Value::from_struct_object(page_context.clone());

                Ok(wurbo::jinja::render(
                    &target,
                    templates,
                    ctx,
                    wurbo_tracker::track,
                )?)
            }

            fn activate(selectors: Option<Vec<String>>) {
                let listeners = LISTENERS_MAP.lock().unwrap();
                for (selector, ty) in listeners.iter() {
                    // if selectors is Some, then only activate the listeners that match
                    if let Some(selectors) = &selectors {
                        if !selectors.contains(&selector.to_string()) {
                            continue;
                        }
                    }
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

    #[test]
    fn smoke() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

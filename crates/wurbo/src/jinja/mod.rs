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
use std::sync::LazyLock;
use std::sync::Mutex;

/// Use LazyLock and Mutex static to hold the latest Environment settings. Can be set by using the initialize
/// function.
pub static ENVIRO: LazyLock<Mutex<Environment>> = LazyLock::new(|| Mutex::new(Environment::new()));

/// This struct and it's impls replaces the templates Vec with a type that identifies an Entry
/// tuple,
#[derive(Debug, Clone, Default)]
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
    pub fn new(name: String, template: String) -> Self {
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

#[derive(Debug, Clone, Default)]
pub struct Entry {
    pub name: String,
    pub template: String,
}

impl Entry {
    pub fn new(name: String, template: String) -> Self {
        Self { name, template }
    }
}

/// Impl IntoIterator for Templates
impl IntoIterator for Templates {
    type Item = (String, String);
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

/// Initialize the Environment with the given templates. Use default filters.
pub fn customize<T>(
    templates: T,
    tracker: fn(String, String) -> String,
) -> Result<(), error::RenderError>
where
    T: IntoIterator<Item = (String, String)> + std::fmt::Debug,
{
    // Get the env from ENV
    let mut env = ENVIRO.lock().unwrap();

    // Convert the
    for (name, template) in templates.into_iter() {
        env.add_template_owned(name, template)?;
    }

    env.add_filter("on", tracker);

    Ok(())
}

/// Render the given entry filename with the given templates using the context
/// Trackers will register any event listeners as per the templates' `on` attributes
pub fn render(
    // Template target for this rendering
    target: &str,
    ctx: Value,
) -> Result<String, error::RenderError> {
    let env = ENVIRO.lock().unwrap();
    let tmpl = env.get_template(target)?;

    let rendered = tmpl.render(&ctx).map_err(|e| {
        // render causes as well
        let mut err = &e as &dyn std::error::Error;
        while let Some(next_err) = err.source() {
            err = next_err;
        }
        error::RenderError::from(e)
    })?;
    Ok(rendered)
}

/// Implements the wurbo_out::Guest trait for the given component.
///
/// You must have a [`PageContext`] struct that implements [`minijinja::value::Object`] in the same file where you use
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

        ///Maps the #elementId to the event type
        type ListenerMap = std::collections::HashMap<String, String>;

        // We cannot have &self in the Component struct,
        // so we use static variables to store the state between functions
        // See https://crates.io/crates/lazy_static
        lazy_static! {
        // create Vec<bindings::component::cargo_comp::imports::ListenDetails>
        static ref LISTENERS_MAP: std::sync::Mutex<ListenerMap> = std::sync::Mutex::new(std::collections::HashMap::new());
        // cache the last state of $pagecontext as Mutex
        static ref $state: std::sync::Mutex<Option<$pagecontext>> = std::sync::Mutex::new(None);
        }

        /// The default templates get loaded once, and then the user can customize them thereafter. These
        /// default templates are kept in this static variable.
        pub static DEFAULT_TEMPLATES: std::sync::OnceLock<Templates> = std::sync::OnceLock::new();

        /// unique namespace to clairfy and avoid collisions with other Guest code
        mod wurbo_tracker {
            /// Insert the source element id, event type, and target output id into the LISTENERS_MAP
            pub fn track(elem_id: String, ty: String) -> String {
                let mut listeners = super::LISTENERS_MAP.lock().unwrap();

                // print warning if elem_id string is empty for listener type 'ty'
                if elem_id.is_empty() {
                    eprintln!("WARNING: elem_id is empty for listener type {ty}");
                } else {
                    listeners.insert(format!("#{elem_id}"), ty);
                }

                elem_id
            }
        }

        impl $guest for $component {
            /// Optionally load an array of custom (name, string) templates into the minijinja Environment
            ///
            /// If this is called before the first render, ensure you customize **all** the templates
            /// provided as defaults in the get_templates() function,
            /// otherwise the default templates will be used.
            ///
            /// If called after the first render, any template can be customized individually.
            fn customize(templates: Vec<(String, String)>) -> Result<(), String> {
                $crate::jinja::customize(templates, wurbo_tracker::track).map_err(|e| e.to_string())
            }

            /// Renders the given context into the target template, and returns the rendered HTML
            fn render(context: $context) -> Result<String, String> {
                // First, enure that the default templates are loaded
                let templates = DEFAULT_TEMPLATES.get_or_init(|| {
                    // get_templates() is provided by the user calling tis macro
                    let tmpls = get_templates();

                    // Try to get_template of entry.name and output.name,
                    // if either is not found, then call customize using default templates to load
                    // them into ENVIRO
                    let env = $crate::jinja::ENVIRO.lock().unwrap();
                    if let Err(_) = env
                        .get_template(&tmpls.entry.name)
                        .and(env.get_template(&tmpls.output.name))
                    {
                        drop(env);
                        $crate::jinja::customize(tmpls.clone(), wurbo_tracker::track)
                            .expect("Should be able to load Default templates");
                    };
                    tmpls
                });

                let page_context = $pagecontext::from(&context);
                // update cache
                let mut last_state = $state.lock().unwrap();
                // templates

                let target = if let Some(ref target) = page_context.target {
                    // if target is Some, then use that as the target minijinja template
                    target
                } else if last_state.is_none() {
                    // if last_state is None, then use the entry minijinja template
                    &templates.entry.name
                } else {
                    // if last_state is Some, but no override set, then use the output minijinja template
                    &templates.output.name
                };

                *last_state = Some(page_context.clone());

                let ctx = Value::from_object(page_context.clone());

                Ok(wurbo::jinja::render(&target, ctx)?)
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
    // use super::*;

    #[test]
    fn smoke() -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}

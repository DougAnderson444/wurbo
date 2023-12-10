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
use crate::utils;
mod count;
mod error;

use minijinja::value::{StructObject, Value};
use minijinja::Environment;

/// Represents the context for the page
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PageContext {
    page: Page,
    input: Input,
    output: Output,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
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

pub fn render(
    entry: &str,
    templates: &[(&str, &str)],
    ctx_struct: PageContext,
) -> Result<String, error::RenderError> {
    let mut env = Environment::new();

    templates.iter().for_each(|(name, template)| {
        env.add_template(name, template)
            .expect("template should be added");
    });

    let ctx = Value::from_struct_object(ctx_struct);

    let tmpl = env.get_template(entry)?;

    let rendered = tmpl.render(&ctx)?;
    Ok(rendered)
}

#[cfg(test)]
mod jinja_unit_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn smoke() -> Result<(), String> {
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

        let rendered_page = render("page.html", &tmpls, ctx_struct).expect("test should pass");

        eprintln!("rendered_page:\n{}", rendered_page);

        // write rendered_page to file
        std::fs::write(path.join("rendered_page.html"), rendered_page)?;

        eprintln!("\n\nNow, let's render just the Output \n\n");

        let output_ctx = PageContext {
            page: Page {
                title: "Let's count vowels using templates for Inputs and Outputs!".to_string(),
            },
            input: Input {
                placeholder: "Input the word with vowels here".to_string(),
            },
            output: Output {
                name: "vowels".to_string(),
            },
        };

        let output_tmpls = vec![("output.html", include_str!("fixtures/output.html"))];

        let rendered_output =
            render("output.html", &output_tmpls, output_ctx).expect("test should pass");

        eprintln!("OUTPUT ONLY:\n{}", rendered_output);

        Ok(())
    }
}

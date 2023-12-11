/// Errors that can occur when rendering a template
pub struct RenderError {
    source: minijinja::Error,
}

impl From<minijinja::Error> for RenderError {
    fn from(source: minijinja::Error) -> Self {
        Self { source }
    }
}

impl From<RenderError> for String {
    fn from(e: RenderError) -> Self {
        e.to_string()
    }
}

/// impl Display for RenderError
impl std::fmt::Display for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&stringify_errors(&self.source))
    }
}

/// Use stringify_errors to Display the full error trail
impl std::fmt::Debug for RenderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&stringify_errors(&self.source))
    }
}

/// Rolls up all the errors into a single string
fn stringify_errors(e: &dyn std::error::Error) -> String {
    let mut err = e as &dyn std::error::Error;
    let mut response = String::new();
    while let Some(next_err) = err.source() {
        response.push_str(&format!("\n caused by: {:#}", next_err));
        err = next_err;
    }
    response
}

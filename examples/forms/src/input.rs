use super::*;

/// Input is the input form(s)
#[derive(Debug, Clone)]
pub(crate) struct Input(types::Input);

impl StructObject for Input {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "id" => Some(Value::from(rand_id())),
            "placeholder" => Some(Value::from(self.placeholder.clone())),
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

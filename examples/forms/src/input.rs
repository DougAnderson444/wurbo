use super::*;

use std::sync::Arc;

/// Input is the input form(s)
#[derive(Debug, Clone)]
pub(crate) struct Input(types::Input);

impl Object for Input {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "id" => Some(Value::from(rand_id())),
            "placeholder" => Some(Value::from(self.placeholder.clone())),
            _ => None,
        }
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

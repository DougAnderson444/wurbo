use super::*;

/// Page is the wrapper for Input and Output
#[derive(Debug, Clone)]
pub(crate) struct Page(types::Page);

impl Object for Page {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "title" => Some(Value::from(self.title.clone())),
            "id" => Some(Value::from(rand_id())),
            _ => None,
        }
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

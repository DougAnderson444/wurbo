use super::*;

/// Page is the wrapper for Input and Output
#[derive(Debug, Clone)]
pub(crate) struct Page(types::Page);

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

use super::*;

/// Output handles the storage of the values and the calculation of the length of the concatenated
#[derive(Debug, Default, Clone)]
pub(super) struct Output {
    id: Option<String>,
    pub(crate) username: Username,
    pub(crate) password: Password,
}

impl Output {
    /// Calculate the length of the username and password concatenated
    fn calculate(&self) -> Value {
        Value::from(*&self.concat().len())
    }

    /// Concatenate the username and password
    fn concat(&self) -> String {
        format!(
            "{}{}",
            self.username
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or_default(),
            self.password
                .as_ref()
                .map(|v| v.clone())
                .unwrap_or_default()
        )
    }
}

/// Impleent Object for Output so we can use minijina to automagically calculate the length
/// of the username and password concatenated
impl Object for Output {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "username" => Some(Value::from_object(self.username.clone())),
            "password" => Some(Value::from_object(self.password.clone())),
            "value" => Some(Value::from(self.concat())),
            // self.username.value
            "count" => Some(Value::from(self.calculate())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(rand_id()))),
            _ => None,
        }
    }
}

/// Username captures is the username input value.
#[derive(Debug, Default, Clone)]
pub(crate) struct Username(Option<String>);

impl Object for Username {
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref()
                    .as_ref()
                    .map(|v| v.clone())
                    .unwrap_or_default(),
            )),
            _ => None,
        }
    }
}

impl From<String> for Username {
    fn from(context: String) -> Self {
        Username(Some(context.clone()))
    }
}

impl From<Option<String>> for Username {
    fn from(context: Option<String>) -> Self {
        Username(context)
    }
}

impl Deref for Username {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [Password] is the password field in the form
/// We wrap it as a newtype so that we can impl [Object] for it
/// We impl [Deref] so we can access the inner of the Rust smart pointer
#[derive(Debug, Default, Clone)]
pub(crate) struct Password(Option<String>);

impl Object for Password {
    // If you add fields to the , you'd add them also here below:
    fn get_value(self: &Arc<Self>, key: &Value) -> Option<Value> {
        match key.as_str()? {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref()
                    .as_ref()
                    .map(|v| v.clone())
                    .unwrap_or_default(),
            )),
            _ => None,
        }
    }
}

impl From<String> for Password {
    fn from(context: String) -> Self {
        Password(Some(context.clone()))
    }
}

impl From<Option<String>> for Password {
    fn from(context: Option<String>) -> Self {
        Password(context)
    }
}

impl Deref for Password {
    type Target = Option<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

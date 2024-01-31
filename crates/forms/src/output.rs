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
                .map(|v| v.value.clone())
                .unwrap_or_default(),
            self.password
                .as_ref()
                .map(|v| v.value.clone())
                .unwrap_or_default()
        )
    }
}

/// Impleent StructObject for Output so we can use minijina to automagically calculate the length
/// of the username and password concatenated
impl StructObject for Output {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "username" => Some(Value::from_struct_object(self.username.clone())),
            "password" => Some(Value::from_struct_object(self.password.clone())),
            "value" => Some(Value::from(self.concat())),
            // self.username.value
            "count" => Some(Value::from(self.calculate())),
            // if self.id.is_some, use it, otherwise generate a new one
            "id" => Some(Value::from(self.id.clone().unwrap_or(rand_id()))),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value", "count", "id", "username", "password"])
    }
}

/// Username captures is the username input value.
#[derive(Debug, Default, Clone)]
pub(crate) struct Username(Option<types::Outrecord>);

impl StructObject for Username {
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref().map(|v| v.value.clone()).unwrap_or_default(),
            )),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value"])
    }
}

impl From<&types::Outrecord> for Username {
    fn from(context: &types::Outrecord) -> Self {
        Username(Some(context.clone()))
    }
}

impl From<Option<types::Outrecord>> for Username {
    fn from(context: Option<types::Outrecord>) -> Self {
        Username(context)
    }
}

impl Deref for Username {
    type Target = Option<types::Outrecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// [Password] is the password field in the form
/// We wrap it as a newtype so that we can impl [StructObject] for it
/// We impl [Deref] so we can access the inner of the Rust smart pointer
#[derive(Debug, Default, Clone)]
pub(crate) struct Password(Option<types::Outrecord>);

impl StructObject for Password {
    // If you add fields to the Outrecord, you'd add them also here below:
    fn get_field(&self, name: &str) -> Option<Value> {
        match name {
            "value" => Some(Value::from(
                // Deref self and use value if is_Some, otherwise use ""
                self.as_ref().map(|v| v.value.clone()).unwrap_or_default(),
            )),
            _ => None,
        }
    }

    /// So that debug will show the values
    fn static_fields(&self) -> Option<&'static [&'static str]> {
        Some(&["value"])
    }
}

impl From<&types::Outrecord> for Password {
    fn from(context: &types::Outrecord) -> Self {
        Password(Some(context.clone()))
    }
}

impl From<Option<types::Outrecord>> for Password {
    fn from(context: Option<types::Outrecord>) -> Self {
        Password(context)
    }
}

impl Deref for Password {
    type Target = Option<types::Outrecord>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

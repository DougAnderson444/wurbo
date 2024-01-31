use serde::Deserialize;

/// Crate Error types
#[derive(Debug, Deserialize)]
pub enum Error {
    /// Error from serde_json
    SerdeJson,
    /// base64ct error
    Base64,
}

impl From<serde_json::Error> for Error {
    fn from(_: serde_json::Error) -> Self {
        Error::SerdeJson
    }
}

impl From<base64ct::Error> for Error {
    fn from(_: base64ct::Error) -> Self {
        Error::Base64
    }
}

/// to string
impl ToString for Error {
    fn to_string(&self) -> String {
        match self {
            Error::SerdeJson => "Serde JSON Error".to_string(),
            Error::Base64 => "Base64 Error".to_string(),
        }
    }
}

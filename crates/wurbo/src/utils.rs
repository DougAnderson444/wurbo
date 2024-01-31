use base64ct::{Base64UrlUnpadded, Encoding};
use convert_case::{Case, Casing};
use nanoid::nanoid;
use serde::{de::Deserialize, Serialize};
use std::fmt::Debug;

use crate::error;

/// CSS Selector IDs need to start with a letter, so we can't use the default nanoid alphabet.
/// We'll use this custom one instead.
const ALPHABET: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

/// Generate a random ID for a CSS selector
pub fn rand_id() -> String {
    let id = nanoid!(16, &ALPHABET);
    id
}

/// A function that prints out any type and returns the first variant in Kebab case
pub fn variant_string<T: Debug>(t: T) -> String {
    let printed = format!("{:?}", t);
    let parts = printed
        .split(|c: char| !c.is_alphanumeric())
        .collect::<Vec<_>>();
    parts[2].to_case(Case::Kebab)
}

/// Decode the base64 string, then deserialize the bytes into JSON [serde_json::Value].
pub fn from_urlsafe(base64: &str) -> Result<serde_json::Value, error::Error> {
    let decoded = Base64UrlUnpadded::decode_vec(base64)?;
    serde_json::from_slice(&decoded)?
}

/// A Trait that does both serde_json and Base64UrlUnpadded encoding
pub trait Base64JSON {
    /// Function that serializes the Context to bytes (serde_json) then encodes it as base64.
    fn to_urlsafe(&self) -> Result<String, error::Error>
    where
        Self: Serialize,
    {
        let serialized = serde_json::to_string(&self)?;
        Ok(Base64UrlUnpadded::encode_string(serialized.as_bytes()))
    }

    /// Decode the base64 string, then deserialize the bytes into [Self].
    fn from_urlsafe(base64: &str) -> Result<Self, error::Error>
    where
        Self: for<'a> Deserialize<'a>,
    {
        let decoded = Base64UrlUnpadded::decode_vec(base64)?;
        serde_json::from_slice(&decoded)?
    }
}

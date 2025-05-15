use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

/// Represents the name of a key to be used for searching
/// 
/// Can be either a single string or an array of strings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuseOptionKeyName<'a> {
    /// A single string representing a key name
    String(Cow<'a, str>),
    /// An array of strings representing nested path components
    StringArray(Vec<Cow<'a, str>>),
}

/// Function type for retrieving values from a JSON structure
pub type FuseKeyValueGetter = Option<fn(&Value) -> &str>;

/// A complex key configuration with name and optional weight
/// 
/// This allows you to specify the importance of certain keys in the search
/// by giving them different weights.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseOptionKeyObject<'a> {
    /// The name of the key to search within
    pub name: Cow<'a, FuseOptionKeyName<'a>>, // Use Cow to avoid cloning
    /// Optional weight to give to matches found in this key (default: 1.0)
    pub weight: Option<f64>,

    /// Custom function to extract values for this key
    #[serde(skip)]
    pub get_fn: FuseKeyValueGetter,
}

/// Defines which keys in the data to search
/// 
/// This can be a single string key, an array of keys, or a complex key object
/// that includes weights for different fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuseOptionKey<'a> {
    /// A complex key configuration with name and optional weight
    KeyObject(FuseOptionKeyObject<'a>),
    /// A single string key name
    String(Cow<'a, str>),
    /// An array of string key names to search within
    StringArray(Vec<Cow<'a, str>>),
}

impl<'a> Default for FuseOptionKey<'a> {
    fn default() -> Self {
        Self::StringArray(Vec::new())
    }
}

//! Key definitions for configuring searchable fields
//!
//! This module provides types and utilities for defining which fields
//! in your documents should be searched, and how they should be weighted
//! in relevance calculations.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;

//----------------------------------------------------------------------
// Key Definition Types
//----------------------------------------------------------------------

/// Represents the name of a key to be used for searching
/// 
/// Can be either a single string or an array of strings to represent
/// nested paths within a document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FuseOptionKeyName<'a> {
    /// A single string representing a key name (e.g., "title")
    String(Cow<'a, str>),
    
    /// An array of strings representing nested path components (e.g., ["user", "profile", "name"])
    StringArray(Vec<Cow<'a, str>>),
}

impl<'a> FuseOptionKeyName<'a> {
    /// Check if the key name is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::String(s) => s.is_empty(),
            Self::StringArray(arr) => arr.is_empty() || arr.iter().all(|s| s.is_empty()),
        }
    }
}

/// Function type for retrieving values from a JSON structure
///
/// This allows custom accessor functions to extract values from complex structures.
pub type FuseKeyValueGetter = Option<fn(&Value) -> &str>;

/// A complex key configuration with name and optional weight
/// 
/// This allows you to specify the importance of certain keys in the search
/// by giving them different weights. Higher weights make matches in those
/// fields more significant in the final relevance score.
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
/// This flexible enum can represent various ways of specifying
/// searchable fields in your documents.
///
/// # Examples
///
/// ```
/// use fuse_rs::FuseOptionKey;
///
/// // Simple string keys
/// let key1 = FuseOptionKey::String("title".into());
///
/// // Array of string keys
/// let key2 = FuseOptionKey::StringArray(vec!["author".into(), "bio".into()]);
///
/// // These keys would be used in FuseOptions configuration
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuseOptionKey<'a> {
    /// A complex key configuration with name and optional weight
    KeyObject(FuseOptionKeyObject<'a>),
    
    /// A single string key name (e.g., "title")
    String(Cow<'a, str>),
    
    /// An array of string key names to search within
    StringArray(Vec<Cow<'a, str>>),
}

impl<'a> Default for FuseOptionKey<'a> {
    fn default() -> Self {
        Self::StringArray(Vec::new())
    }
}

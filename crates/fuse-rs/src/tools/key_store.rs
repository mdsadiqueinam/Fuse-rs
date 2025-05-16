use std::borrow::Cow;
use std::collections::HashMap;
use serde::Serialize;
use crate::core::options::keys::{FuseOptionKey, FuseOptionKeyName, FuseOptionKeyObject, FuseKeyValueGetter};

//----------------------------------------------------------------------
// Key and KeyStore Implementation
//----------------------------------------------------------------------

/// A key object representing a searchable field within a document.
/// 
/// Each Key represents a specific path or field in the document data
/// that should be searched and scored during the fuzzy search process.
/// 
/// # Example
/// 
/// ```
/// // Internally created from FuseOptionKey definitions
/// // Example path: ["author", "name"]
/// // Example id: "author.name"
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Key<'a> {
    /// The field path components to access the data
    pub path: Vec<String>,

    /// A unique identifier for the key (dot-joined path)
    pub id: String,

    /// The weight of the key for scoring calculations (normalized)
    pub weight: f64,

    /// The original source path from which the key was created
    pub src: Cow<'a, str>,

    /// Function to retrieve values from the target document
    #[serde(skip)]
    pub get_fn: FuseKeyValueGetter,
}

/// A container and manager for a collection of searchable `Key` objects.
/// 
/// The `KeyStore` handles normalization of key weights, provides lookup by ID,
/// and manages serialization of the key collection.
///
/// # Example
///
#[derive(Debug, Clone, Serialize)]
pub struct KeyStore<'a> {
    /// All searchable keys in the collection
    keys: Vec<Key<'a>>,

    /// Fast lookup map from key ID to the key object
    #[serde(skip)]
    key_map: HashMap<String, Key<'a>>,
}

impl<'a> KeyStore<'a> {
    /// Creates a new `KeyStore` from a slice of `FuseOptionKey` definitions.
    ///
    /// This normalizes the key weights so that their total equals `1.0`.
    ///
    /// # Arguments
    ///
    /// * `keys` - A slice of `FuseOptionKey` which can be strings, arrays, or objects.
    ///
    /// # Panics
    ///
    /// Panics if any provided key object has a weight less than or equal to zero.
    pub fn new(keys: &[FuseOptionKey<'a>]) -> Self {
        let mut raw_keys: Vec<Key<'a>> = Vec::with_capacity(keys.len());
        let mut total_weight = 0.0;

        for key in keys {
            let key_obj = create_key(key);
            total_weight += key_obj.weight;
            raw_keys.push(key_obj);
        }

        let normalize = |w: f64| if total_weight > 0.0 { w / total_weight } else { w };

        let keys: Vec<Key<'a>> = raw_keys
            .into_iter()
            .map(|mut k| {
                k.weight = normalize(k.weight);
                k
            })
            .collect();

        let key_map = keys.iter().cloned().map(|k| (k.id.clone(), k)).collect();

        Self { keys, key_map }
    }

    /// Retrieves a key by its identifier.
    ///
    /// # Arguments
    ///
    /// * `key_id` - A string slice representing the key ID.
    ///
    /// # Returns
    ///
    /// An `Option` containing the reference to the `Key` if found.
    pub fn get(&self, key_id: &str) -> Option<&Key<'a>> {
        self.key_map.get(key_id)
    }

    /// Returns a reference to all stored keys.
    pub fn keys(&self) -> &[Key<'a>] {
        &self.keys
    }

    /// Serializes the key store into a JSON array string.
    ///
    /// # Returns
    ///
    /// `Result<String, serde_json::Error>` containing the JSON representation of the key array.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self.keys)
    }
}

/// Creates a `Key` object from a `FuseOptionKey`.
///
/// # Arguments
///
/// * `key` - A reference to a `FuseOptionKey` enum.
///
/// # Returns
///
/// A `Key` instance with parsed and derived metadata.
fn create_key<'a>(key: &FuseOptionKey<'a>) -> Key<'a> {
    let (src, path): (Cow<str>, Vec<String>);
    let mut weight = 1.0;
    let mut get_fn = None;

    match key {
        FuseOptionKey::String(s) => {
            src = s.clone();
            path = create_key_path(s);
        },
        FuseOptionKey::StringArray(arr) => {
            path = arr.iter().map(|s| s.to_string()).collect();
            src = Cow::Owned(path.join("."));
        },
        FuseOptionKey::KeyObject(obj) => {
            match &*obj.name {
                FuseOptionKeyName::String(name) => {
                    src = name.clone();
                    path = create_key_path(name);
                },
                FuseOptionKeyName::StringArray(arr) => {
                    path = arr.iter().map(|s| s.to_string()).collect();
                    src = Cow::Owned(path.join("."));
                }
            }

            if let Some(w) = obj.weight {
                if w <= 0.0 {
                    panic!("Invalid weight ({}) for key: '{}'", w, path.join("."));
                }
                weight = w;
            }

            get_fn = obj.get_fn;
        }
    }

    let id = create_key_id(&path);

    Key { path, id, weight, src, get_fn }
}

/// Converts a dotted key string into a vector of path components.
///
/// # Arguments
///
/// * `key` - A dot-delimited string (e.g., `"author.name"`).
///
/// # Returns
///
/// A `Vec<String>` of path components.
pub fn create_key_path(key: &str) -> Vec<String> {
    key.split('.').map(str::to_owned).collect()
}

/// Generates a key ID by joining path components with a dot.
///
/// # Arguments
///
/// * `path` - A slice of strings representing path components.
///
/// # Returns
///
/// A `String` representing the dot-joined key ID.
pub fn create_key_id(path: &[String]) -> String {
    path.join(".")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_key_store_creation() {
        let keys = vec![
            FuseOptionKey::String(Cow::Borrowed("name")),
            FuseOptionKey::StringArray(vec![Cow::Borrowed("author"), Cow::Borrowed("name")]),
            FuseOptionKey::KeyObject(FuseOptionKeyObject {
                name: Cow::Borrowed(&FuseOptionKeyName::String(Cow::Borrowed("title"))),
                weight: Some(2.0),
                get_fn: None,
            }),
        ];

        let key_store = KeyStore::new(&keys);

        assert_eq!(key_store.keys().len(), 3);

        let total_weight: f64 = key_store.keys().iter().map(|k| k.weight).sum();
        assert!((total_weight - 1.0).abs() < EPSILON);

        let title_key = key_store.get("title").unwrap();
        assert_eq!(title_key.src, "title");
        assert!(title_key.weight > 0.0);
    }
}

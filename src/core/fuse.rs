use serde_json::Value;
use crate::{core::options::config::FuseOptions, tools::key_store::KeyStore};

pub struct Fuse<'a> {
    // The options for the Fuse instance
    options: FuseOptions<'a>,
    // The data to be searched
    data: Vec<Value>,

    key_store: KeyStore<'a>
}

impl<'a> Fuse<'a> {
    /// Creates a new Fuse instance with the given options and data
    pub fn new(data: &[Value], options: &FuseOptions<'a>) -> Self {
        let cloned_options = options.clone();
        let key_store = KeyStore::new(&cloned_options.keys);

        Fuse {
            options: cloned_options,
            data: data.to_vec(),
            key_store
        }
    }

    /// Searches the data using the provided search term
    pub fn search(&self, _term: &str) -> Vec<Value> {
        // TODO: Implement actual fuzzy search logic
        // Currently returns an empty vector as a placeholder
        vec![]
    }
}
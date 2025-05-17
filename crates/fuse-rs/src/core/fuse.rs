use crate::{
    core::{options::config::FuseOptions, error_messages::FuseError},
    tools::{fuse_index::FuseIndex, key_store::KeyStore},
};
use serde_json::Value;

//----------------------------------------------------------------------
// Main Fuse Implementation
//----------------------------------------------------------------------

/// The primary struct for fuzzy searching functionality.
///
/// `Fuse` provides methods to perform fuzzy searches on a collection of JSON values
/// using configurable options for matching and scoring.
///
/// # Example
///
pub struct Fuse<'a> {
    /// Configuration options for search behavior
    options: FuseOptions<'a>,

    /// The collection of documents to search through
    docs: Vec<Value>,

    /// Index structure for searchable keys in documents
    key_store: KeyStore<'a>,

    index: FuseIndex<'a>,
}

impl<'a> Fuse<'a> {
    /// Creates a new Fuse instance with the given data and search options.
    ///
    /// # Arguments
    ///
    /// * `data` - A slice of JSON values to search through
    /// * `options` - Configuration options for search behavior
    ///
    /// # Returns
    ///
    /// A new `Fuse` instance ready to perform searches
    pub fn new(docs: &[Value], options: &FuseOptions<'a>, index: Option<FuseIndex<'a>>) -> Self {
        let cloned_options = options.clone();
        let key_store = KeyStore::new(&cloned_options.keys);
        let fuse_index = if let Some(f_index) = index {
            f_index
        } else {
            FuseIndex::create_index(
                &cloned_options.keys,
                &docs,
                Some(cloned_options.get_fn),
                Some(cloned_options.field_norm_weight),
            )
        };

        Fuse {
            options: cloned_options,
            docs: docs.to_vec(),
            key_store,
            index: fuse_index,
        }
    }

    /// Searches the data using the provided search term.
    ///
    /// # Arguments
    ///
    /// * `term` - The search pattern to look for
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of matching JSON values sorted by relevance,
    /// or an error if the search cannot be performed.
    pub fn search(&self, term: &str) -> Result<Vec<Value>, FuseError> {
        
        // TODO: Implement actual fuzzy search logic
        // Currently returns an empty vector as a placeholder
        Ok(vec![])
    }

}

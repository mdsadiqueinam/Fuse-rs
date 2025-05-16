//! Search index implementation for improved search performance
//!
//! This module provides the indexing functionality that speeds up
//! fuzzy searches by pre-processing the data collection.

use std::collections::HashMap;

use serde_json::Value;

use crate::{helpers::get::GetFn, FuseOptions};
use super::key_store::Key;
use super::norm::Norm;
use super::fuse_index_record::FuseIndexRecords;

//----------------------------------------------------------------------
// Types & Constants
//----------------------------------------------------------------------

/// Default size for the n-gram indexing
const DEFAULT_NGRAM_SIZE: usize = 3;

/// Search index for fast fuzzy search operations
///
/// This structure maintains an inverted index mapping tokens to document IDs,
/// which allows for faster search operations compared to linear scanning.
#[derive(Debug)]
pub struct FuseIndex<'a> {
    norm: Norm,
    get_fn: GetFn,
    is_created: bool,
    records: FuseIndexRecords,
    docs: Vec<Value>,
    keys: Vec<Key<'a>>,
    keys_map: HashMap<String, usize>,
}

//----------------------------------------------------------------------
// Implementation
//----------------------------------------------------------------------

impl<'a> FuseIndex<'a> {
   pub fn new(options: &FuseOptions) -> Self {
        FuseIndex {
            norm: Norm::new(options.field_norm_weight, 3),
            get_fn: options.get_fn,
            is_created: false,
            records: FuseIndexRecords::new(),
            docs: Vec::new(),
            keys: Vec::new(),
            keys_map: HashMap::new(),
        }
    }

    pub fn set_source(&mut self, source: Vec<Value>) {
        self.docs = source;
    }

    pub fn set_index_records(&mut self, records: FuseIndexRecords) {
        self.records = records;
    }

    pub fn set_keys(&mut self, keys: Vec<Key<'a>>) {
        self.keys = keys;
        self.keys_map = self.keys
            .iter()
            .enumerate()
            .map(|(i, key)| (key.id.clone(), i))
            .collect();
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn test_index_creation() {
        // TODO: Implement tests when the actual index is implemented
    }
}
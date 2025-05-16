//! Search index implementation for improved search performance
//!
//! This module provides the indexing functionality that speeds up
//! fuzzy searches by pre-processing the data collection.

use std::borrow::Cow;
use std::collections::HashMap;

use serde_json::Value;

use super::fuse_index_record::{
    FuseIndexObjectRecord, FuseIndexRecords, FuseIndexRecordsVecExt, FuseIndexStringRecord, IndexValue, RecordEntry, RecordEntryValue
};
use super::key_store::Key;
use super::norm::Norm;
use crate::helpers::get::{GetFnPath, GetValue};
use crate::{FuseOptions, helpers::get::GetFn};

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
        self.keys_map = self
            .keys
            .iter()
            .enumerate()
            .map(|(i, key)| (key.id.clone(), i))
            .collect();
    }

    fn add(&mut self, doc: &Value) {
        let idx = self.docs.len();

        if doc.is_string() {
            self.add_string(doc, idx);
        } else {
            self.add_object(doc, idx);
        }
    }

    fn add_string(&mut self, doc: &Value, idx: usize) {
        if let Some(value) = doc.as_str() {
            if value.is_empty() {
                return;
            }

            let norm = self.norm.get(value);
            let record = FuseIndexStringRecord::new(idx, value.to_string(), norm);
            self.records.add_string(record);
        }
    }

    fn add_object(&mut self, doc: &Value, idx: usize) {
        let mut record = FuseIndexObjectRecord::new(idx);

        self.keys.iter().enumerate().for_each(|(keyIndex, key)| {
            let get_value = if let Some(get_fn) = key.get_fn {
                Some(GetValue::String(get_fn(doc).to_string()))
            } else {
                let path: Vec<Cow<'_, str>> =
                    key.path.iter().map(|s| Cow::Borrowed(s.as_str())).collect();
                let get_fn_path = GetFnPath::StringArray(path);
                (self.get_fn)(doc, &get_fn_path)
            };

            if let Some(value) = get_value {
                match value {
                    GetValue::String(s) => {
                        let norm = self.norm.get(&s);
                        let entry = RecordEntryValue::Single(IndexValue {
                            v: s,
                            n: norm,
                            i: None,
                        });
                        record.entries.insert(keyIndex.to_string(), entry);
                    }
                    GetValue::Array(arr) => {
                        
                    }
                }
            } else {
                return;
            }
        });

        self.records.add_object(record);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_index_creation() {
        // TODO: Implement tests when the actual index is implemented
    }
}

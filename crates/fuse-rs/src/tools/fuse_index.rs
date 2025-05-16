//! Search index implementation for improved search performance
//!
//! This module provides the indexing functionality that speeds up
//! fuzzy searches by pre-processing the data collection.

use std::borrow::Cow;
use std::collections::HashMap;

use serde_json::Value;

use super::fuse_index_record::*;
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
    records: FuseIndexRecords,
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
            records: FuseIndexRecords::new(),
            keys: Vec::new(),
            keys_map: HashMap::new(),
        }
    }

    pub fn set_source(&mut self, source: Vec<Value>) {
        // Clear existing records and documents
        self.records.clear();

        source.iter().for_each(|doc| {
            self.add(doc);
        });
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

    pub fn add(&mut self, doc: &Value) {
        // add a new record at the end of the records
        let idx = self.size();

        if doc.is_string() {
            self.add_string(doc, idx);
        } else {
            self.add_object(doc, idx);
        }
    }

    pub fn removeAt(&mut self, idx: usize) {
        // Remove the record at the specified index
        self.records.remove(idx);

        // Update the index of all records after the removed index
        for i in idx..self.size() {
            let record = self.records.get_mut(i).unwrap();
            match record {
                FuseIndexRecord::String(r) => r.i -= 1,
                FuseIndexRecord::Object(r) => r.i -= 1,
            };
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

        self.keys.iter().enumerate().for_each(|(key_index, key)| {
            let get_value = self.get_value_for_key(doc, key);

            if let Some(value) = get_value {
                match value {
                    GetValue::String(s) => {
                        self.process_string_value(s, key_index, &mut record);
                    }
                    GetValue::Array(arr) => {
                        self.process_array_value(arr, key_index, &mut record);
                    }
                }
            }
        });

        self.records.add_object(record);
    }

    /// Get the value for a specific key from a document
    fn get_value_for_key(&self, doc: &Value, key: &Key) -> Option<GetValue> {
        if let Some(get_fn) = key.get_fn {
            Some(GetValue::String(get_fn(doc).to_string()))
        } else {
            let path: Vec<Cow<'_, str>> =
                key.path.iter().map(|s| Cow::Borrowed(s.as_str())).collect();
            let get_fn_path = GetFnPath::StringArray(path);
            (self.get_fn)(doc, &get_fn_path)
        }
    }

    /// Process a single string value and add it to the record
    fn process_string_value(
        &self,
        s: String,
        key_index: usize,
        record: &mut FuseIndexObjectRecord,
    ) {
        let norm = self.norm.get(&s);
        let entry = RecordEntryValue::Single(IndexValue {
            v: s,
            n: norm,
            i: None,
        });
        record.entries.insert(key_index.to_string(), entry);
    }

    /// Process an array of values and add them to the record
    fn process_array_value(
        &self,
        arr: Vec<String>,
        key_index: usize,
        record: &mut FuseIndexObjectRecord,
    ) {
        let sub_records = self.collect_sub_records(arr);

        if !sub_records.is_empty() {
            let entry = RecordEntryValue::Array(sub_records);
            record.entries.insert(key_index.to_string(), entry);
        }
    }

    /// Collect sub-records from an array of values
    fn collect_sub_records(&self, arr: Vec<String>) -> Vec<IndexValue> {
        let mut sub_records = Vec::new();
        let mut stack = Vec::new();

        // Initialize stack with all array elements (with their indices)
        for (k, item) in arr.iter().enumerate() {
            stack.push((k, item.clone()));
        }

        // Process the stack
        while let Some((nested_arr_index, value)) = stack.pop() {
            // Skip empty values
            if value.is_empty() {
                continue;
            }

            // Process string values
            let norm = self.norm.get(&value);
            let sub_record = IndexValue {
                v: value,
                n: norm,
                i: Some(nested_arr_index),
            };
            sub_records.push(sub_record);
        }

        sub_records
    }

    fn size(&self) -> usize {
        self.records.len()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_index_creation() {
        // TODO: Implement tests when the actual index is implemented
    }
}

//! Definitions for the fuzzy search index records
//!
//! This module provides data structures for representing indexed records
//! used by the search index to speed up fuzzy searches.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

//----------------------------------------------------------------------
// Types and Implementations
//----------------------------------------------------------------------

/// An entry in the record that contains the value and field-length norm
///
/// # Example (JSON representation)
/// ```json
/// {
///   "v": "Old Man's War",
///   "n": 0.5773502691896258
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexValue {
    /// The text value
    pub v: String,
    /// The field-length norm
    pub n: f64,
    /// Optional index, used in arrays of values
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i: Option<usize>,
}

/// Entry in a record, which can be a single value or an array of values
///
/// # Example (JSON representation for Array variant)
/// ```json
/// [
///   {
///     "v": "pizza lover",
///     "i": 2,
///     "n": 0.7071067811865475
///   },
///   {
///     "v": "hello world",
///     "i": 0,
///     "n": 0.7071067811865475
///   }
/// ]
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordEntryValue {
    /// A single indexed value
    Single(IndexValue),
    /// An array of indexed values
    Array(Vec<IndexValue>),
}

/// Represents the record entries, keyed by the field index
pub type RecordEntry = HashMap<String, RecordEntryValue>;

/// Record for an object with potentially multiple fields that are indexed
///
/// # Example (JSON representation)
/// ```json
/// {
///   "i": 0,
///   "$": {
///     "0": { "v": "Old Man's War", "n": 0.5773502691896258 },
///     "1": { "v": "Codenar", "n": 1 },
///     "2": [
///       { "v": "pizza lover", "i": 2, "n": 0.7071067811865475 },
///       { "v": "helo wold", "i": 1, "n": 0.7071067811865475 },
///       { "v": "hello world", "i": 0, "n": 0.7071067811865475 }
///     ]
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseIndexObjectRecord {
    /// The index of the record in the source list
    pub i: usize,
    /// The mapped field values
    #[serde(rename = "$")]
    pub entries: RecordEntry,
}

impl FuseIndexObjectRecord {
    /// Create a new object record
    pub fn new(index: usize) -> Self {
        Self {
            i: index,
            entries: HashMap::new(),
        }
    }
    
    /// Add a single value entry
    pub fn add_value(&mut self, key: String, value: String, norm: f64) {
        self.entries.insert(
            key,
            RecordEntryValue::Single(IndexValue {
                v: value,
                n: norm,
                i: None,
            }),
        );
    }
    
    /// Add an array value entry
    pub fn add_array(&mut self, key: String, values: Vec<(String, usize, f64)>) {
        let values = values
            .into_iter()
            .map(|(value, index, norm)| IndexValue {
                v: value,
                n: norm,
                i: Some(index),
            })
            .collect();
        
        self.entries.insert(key, RecordEntryValue::Array(values));
    }
}

/// Record for a simple string value
///
/// # Example (JSON representation)
/// ```json
/// {
///   "i": 0,
///   "v": "one",
///   "n": 1
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseIndexStringRecord {
    /// The index of the record in the source list
    pub i: usize,
    /// The text value
    pub v: String,
    /// The field-length norm
    pub n: f64,
}

impl FuseIndexStringRecord {
    /// Create a new string record
    pub fn new(index: usize, value: String, norm: f64) -> Self {
        Self {
            i: index,
            v: value,
            n: norm,
        }
    }
}

/// Union type for different types of records in the index
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuseIndexRecord {
    /// Object record with multiple indexed fields
    Object(FuseIndexObjectRecord),
    /// Simple string record
    String(FuseIndexStringRecord),
}

/// Collection of records in the search index
/// 
/// This can be either a collection of object records or string records,
/// but not a mix of both types.
pub type FuseIndexRecords = Vec<FuseIndexRecord>;

/// Extension trait for adding records to a FuseIndexRecords vector.
pub trait FuseIndexRecordsVecExt {
    /// Adds a string record to the collection. Returns a mutable reference for chaining.
    fn add_string(&mut self, record: FuseIndexStringRecord) -> &mut Self;
    /// Adds an object record to the collection. Returns a mutable reference for chaining.
    fn add_object(&mut self, record: FuseIndexObjectRecord) -> &mut Self;
}

impl FuseIndexRecordsVecExt for FuseIndexRecords {
    fn add_string(&mut self, record: FuseIndexStringRecord) -> &mut Self {
        self.push(FuseIndexRecord::String(record));
        self
    }
    
    fn add_object(&mut self, record: FuseIndexObjectRecord) -> &mut Self {
        self.push(FuseIndexRecord::Object(record));
        self
    }
}

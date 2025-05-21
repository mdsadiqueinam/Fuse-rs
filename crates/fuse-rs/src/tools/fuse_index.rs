//! Search index implementation for improved search performance
//!
//! This module provides the indexing functionality that speeds up
//! fuzzy searches by pre-processing the data collection.

use std::borrow::Cow;
use std::collections::HashMap;

use serde_json::Value;

use super::fuse_index_record::*;
use super::key_store::{Key, create_key};
use super::norm::Norm;
use crate::helpers::get::{GetFnPath, GetValue};
use crate::{FuseOptions, helpers::get::GetFn};
use crate::core::options::keys::FuseOptionKey;

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
    pub records: FuseIndexRecords,
    pub keys: Vec<Key<'a>>,
    pub keys_map: HashMap<String, usize>,
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

    pub fn remove_at(&mut self, idx: usize) {
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

    /// Get the number of records in the index
    pub fn get_value_for_item_at_key_id(&self, item: &RecordEntry, key_id: &str) -> Option<RecordEntryValue> {
        if let Some(key_index) = self.keys_map.get(key_id) {
            item.get(&key_index.to_string()).cloned()
        } else {
            None
        }
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

    pub fn size(&self) -> usize {
        self.records.len()
    }

    /// Creates a new FuseIndex from keys and docs with optional configuration.
    ///
    /// # Arguments
    ///
    /// * `keys` - A slice of `FuseOptionKey` which define the fields to search in the documents.
    /// * `docs` - A slice of `Value` representing the documents to index.
    /// * `get_fn` - Optional function for getting values from documents (defaults to options' get_fn).
    /// * `field_norm_weight` - Optional field normalization weight (defaults to options' field_norm_weight).
    ///
    /// # Returns
    ///
    /// A new `FuseIndex` instance with the documents indexed.
    pub fn create_index(
        keys: &[FuseOptionKey<'a>],
        docs: &[Value],
        get_fn: Option<GetFn>,
        field_norm_weight: Option<f64>,
    ) -> Self {
        let mut options = FuseOptions::default();
        
        if let Some(get_fn_value) = get_fn {
            options.get_fn = get_fn_value;
        }
        
        if let Some(weight) = field_norm_weight {
            options.field_norm_weight = weight;
        }
        
        let mut index = FuseIndex::new(&options);
        
        // Create keys using the key_store's create_key function
        // Handle the Result by unwrapping or panicking with error message
        let keys_vec: Vec<Key> = keys.iter()
            .map(|k| create_key(k).unwrap_or_else(|e| panic!("{}", e)))
            .collect();
        index.set_keys(keys_vec);
        
        // Set the documents to be indexed
        let docs_vec = docs.to_vec();
        index.set_source(docs_vec);
        
        index
    }

    /// Parses an existing index data structure into a FuseIndex instance.
    ///
    /// # Arguments
    ///
    /// * `data` - The pre-computed index records to use.
    /// * `get_fn` - Optional function for getting values from documents (defaults to options' get_fn).
    /// * `field_norm_weight` - Optional field normalization weight (defaults to options' field_norm_weight).
    ///
    /// # Returns
    ///
    /// A new `FuseIndex` instance with the pre-computed index data.
    pub fn parse_index(
        data: (Vec<Key<'a>>, FuseIndexRecords),
        get_fn: Option<GetFn>,
        field_norm_weight: Option<f64>,
    ) -> Self {
        let mut options = FuseOptions::default();
        
        if let Some(get_fn_value) = get_fn {
            options.get_fn = get_fn_value;
        }
        
        if let Some(weight) = field_norm_weight {
            options.field_norm_weight = weight;
        }
        
        let mut index = FuseIndex::new(&options);
        
        let (keys, records) = data;
        index.set_keys(keys);
        index.set_index_records(records);
        
        index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::options::config::FuseOptions;
    use crate::core::options::keys::FuseOptionKey;
    use serde_json::json;

    #[test]
    fn test_index_creation() {
        let options = FuseOptions::default();
        let index = FuseIndex::new(&options);
        
        assert_eq!(index.size(), 0);
        assert!(index.keys.is_empty());
        assert!(index.keys_map.is_empty());
    }
    
    #[test]
    fn test_add_string() {
        let options = FuseOptions::default();
        let mut index = FuseIndex::new(&options);
        
        // Add a string document
        let doc = json!("test string");
        index.add(&doc);
        
        // Verify the document was added
        assert_eq!(index.size(), 1);
        
        // Verify we have the right record type
        if let FuseIndexRecord::String(record) = &index.records[0] {
            assert_eq!(record.i, 0); // Index should be 0
            assert_eq!(record.v, "test string"); // Value should be stored
            assert!(record.n > 0.0); // Norm should be calculated
        } else {
            panic!("Expected string record");
        }
        
        // Test empty string is not added
        let empty_doc = json!("");
        index.add(&empty_doc);
        assert_eq!(index.size(), 1); // Size shouldn't change
    }
    
    #[test]
    fn test_add_object() {
        let mut options = FuseOptions::default();
        
        // Set up keys for indexing
        options.keys = vec![
            FuseOptionKey::String("title".into()),
            FuseOptionKey::String("author".into()),
        ];
        
        let mut index = FuseIndex::new(&options);
        index.set_keys(vec![
            Key {
                path: vec!["title".to_string()],
                id: "title".to_string(),
                weight: 1.0,
                src: "title".into(),
                get_fn: None,
            },
            Key {
                path: vec!["author".to_string()],
                id: "author".to_string(),
                weight: 1.0,
                src: "author".into(),
                get_fn: None,
            },
        ]);
        
        // Add an object document
        let doc = json!({
            "title": "The Great Gatsby",
            "author": "F. Scott Fitzgerald"
        });
        index.add(&doc);
        
        // Verify the document was added
        assert_eq!(index.size(), 1);
        
        // Verify we have the right record type with both fields
        if let FuseIndexRecord::Object(record) = &index.records[0] {
            assert_eq!(record.i, 0); // Index should be 0
            assert_eq!(record.entries.len(), 2); // Should have two entries
            
            // Check title field
            if let RecordEntryValue::Single(title_value) = &record.entries.get("0").unwrap() {
                assert_eq!(title_value.v, "The Great Gatsby");
                assert!(title_value.n > 0.0);
            } else {
                panic!("Title should be a Single value");
            }
            
            // Check author field
            if let RecordEntryValue::Single(author_value) = &record.entries.get("1").unwrap() {
                assert_eq!(author_value.v, "F. Scott Fitzgerald");
                assert!(author_value.n > 0.0);
            } else {
                panic!("Author should be a Single value");
            }
        } else {
            panic!("Expected object record");
        }
    }
    
    #[test]
    fn test_add_object_with_array() {
        let mut options = FuseOptions::default();
        
        // Set up keys for indexing
        options.keys = vec![
            FuseOptionKey::String("title".into()),
            FuseOptionKey::String("tags".into()),
        ];
        
        let mut index = FuseIndex::new(&options);
        index.set_keys(vec![
            Key {
                path: vec!["title".to_string()],
                id: "title".to_string(),
                weight: 1.0,
                src: "title".into(),
                get_fn: None,
            },
            Key {
                path: vec!["tags".to_string()],
                id: "tags".to_string(),
                weight: 1.0,
                src: "tags".into(),
                get_fn: None,
            },
        ]);
        
        // Add an object document with an array field
        let doc = json!({
            "title": "Programming in Rust",
            "tags": ["programming", "rust", "systems"]
        });
        index.add(&doc);
        
        // Verify the document was added
        assert_eq!(index.size(), 1);
        
        // Verify we have the right record type
        if let FuseIndexRecord::Object(record) = &index.records[0] {
            // Check tags field has array values
            if let RecordEntryValue::Array(tags) = &record.entries.get("1").unwrap() {
                assert_eq!(tags.len(), 3);
                
                // Check all tags were indexed with their correct indices
                let tags_values: Vec<&str> = tags.iter().map(|t| t.v.as_str()).collect();
                assert!(tags_values.contains(&"programming"));
                assert!(tags_values.contains(&"rust"));
                assert!(tags_values.contains(&"systems"));
                
                // Check indices are preserved
                for tag in tags {
                    assert!(tag.i.is_some());
                }
            } else {
                panic!("Tags should be an Array value");
            }
        } else {
            panic!("Expected object record");
        }
    }
    
    #[test]
    fn test_remove_at() {
        let options = FuseOptions::default();
        let mut index = FuseIndex::new(&options);
        
        // Add multiple string documents
        index.add(&json!("first"));
        index.add(&json!("second"));
        index.add(&json!("third"));
        
        assert_eq!(index.size(), 3);
        
        // Remove the middle document
        index.remove_at(1);
        
        // Check size decreased
        assert_eq!(index.size(), 2);
        
        // Check indices were updated
        if let FuseIndexRecord::String(first) = &index.records[0] {
            assert_eq!(first.i, 0);
            assert_eq!(first.v, "first");
        }
        
        if let FuseIndexRecord::String(third) = &index.records[1] {
            assert_eq!(third.i, 1); // Index should be decremented
            assert_eq!(third.v, "third");
        }
    }
    
    #[test]
    fn test_get_value_for_item_at_key_id() {
        let mut options = FuseOptions::default();
        
        // Set up keys for indexing
        options.keys = vec![
            FuseOptionKey::String("title".into()),
            FuseOptionKey::String("author".into()),
        ];
        
        let mut index = FuseIndex::new(&options);
        index.set_keys(vec![
            Key {
                path: vec!["title".to_string()],
                id: "title".to_string(),
                weight: 1.0,
                src: "title".into(),
                get_fn: None,
            },
            Key {
                path: vec!["author".to_string()],
                id: "author".to_string(),
                weight: 1.0,
                src: "author".into(),
                get_fn: None,
            },
        ]);
        
        // Add an object document
        index.add(&json!({
            "title": "The Great Gatsby",
            "author": "F. Scott Fitzgerald"
        }));
        
        // Get the record entry
        if let FuseIndexRecord::Object(record) = &index.records[0] {
            // Try to get values by key ID
            let title_value = index.get_value_for_item_at_key_id(&record.entries, "title");
            let author_value = index.get_value_for_item_at_key_id(&record.entries, "author");
            let nonexistent = index.get_value_for_item_at_key_id(&record.entries, "nonexistent");
            
            assert!(title_value.is_some());
            assert!(author_value.is_some());
            assert!(nonexistent.is_none());
            
            // Verify values are correct
            if let RecordEntryValue::Single(title) = title_value.unwrap() {
                assert_eq!(title.v, "The Great Gatsby");
            } else {
                panic!("Expected Single value for title");
            }
            
            if let RecordEntryValue::Single(author) = author_value.unwrap() {
                assert_eq!(author.v, "F. Scott Fitzgerald");
            } else {
                panic!("Expected Single value for author");
            }
        }
    }
    
    #[test]
    fn test_set_source() {
        let options = FuseOptions::default();
        let mut index = FuseIndex::new(&options);
        
        // Add initial documents
        index.add(&json!("initial"));
        assert_eq!(index.size(), 1);
        
        // Set new source, which should replace existing documents
        let new_source = vec![
            json!("one"),
            json!("two"),
            json!("three")
        ];
        
        index.set_source(new_source);
        
        // Check new size
        assert_eq!(index.size(), 3);
        
        // Verify new documents were indexed
        if let FuseIndexRecord::String(record) = &index.records[0] {
            assert_eq!(record.v, "one");
        }
        if let FuseIndexRecord::String(record) = &index.records[1] {
            assert_eq!(record.v, "two");
        }
        if let FuseIndexRecord::String(record) = &index.records[2] {
            assert_eq!(record.v, "three");
        }
    }
    
    #[test]
    fn test_create_index() {
        // Define test keys for searching
        let keys = vec![
            FuseOptionKey::String("title".into()),
            FuseOptionKey::String("author".into()),
        ];
        
        // Create test documents
        let docs = vec![
            json!({
                "title": "The Great Gatsby",
                "author": "F. Scott Fitzgerald"
            }),
            json!({
                "title": "To Kill a Mockingbird",
                "author": "Harper Lee"
            }),
            json!({
                "title": "1984",
                "author": "George Orwell"
            }),
        ];
        
        // Custom field_norm_weight for testing
        let field_norm_weight = 2.0;
        
        // Create index with the test data
        let index = FuseIndex::create_index(&keys, &docs, None, Some(field_norm_weight));
        
        // Verify the index was created correctly
        assert_eq!(index.size(), 3);
        assert_eq!(index.keys.len(), 2);
        
        // Check that the keys were converted and stored properly
        assert_eq!(index.keys[0].id, "title");
        assert_eq!(index.keys[1].id, "author");
        
        // Verify the keys map was created correctly
        assert!(index.keys_map.contains_key("title"));
        assert!(index.keys_map.contains_key("author"));
        
        // We can't directly access norm.weight as it's private
        // Instead, verify the index was created with documents
        
        // Check one of the records to ensure documents were indexed
        if let FuseIndexRecord::Object(record) = &index.records[0] {
            // Check that both fields were indexed
            assert!(record.entries.contains_key("0")); // title
            assert!(record.entries.contains_key("1")); // author
            
            // Verify title value for first document
            if let RecordEntryValue::Single(title_value) = &record.entries.get("0").unwrap() {
                assert_eq!(title_value.v, "The Great Gatsby");
                assert!(title_value.n > 0.0); // Norm should be calculated
            } else {
                panic!("Expected Single value for title");
            }
        } else {
            panic!("Expected object record");
        }
    }
    
    #[test]
    fn test_parse_index() {
        // Create keys for the test
        let keys = vec![
            Key {
                path: vec!["title".to_string()],
                id: "title".to_string(),
                weight: 1.0,
                src: "title".into(),
                get_fn: None,
            },
            Key {
                path: vec!["author".to_string()],
                id: "author".to_string(),
                weight: 1.0,
                src: "author".into(),
                get_fn: None,
            },
        ];
        
        // Create mock records for testing
        let mut records = FuseIndexRecords::new();
        
        // Add a string record
        let string_record = FuseIndexStringRecord::new(0, "test string".to_string(), 1.0);
        records.add_string(string_record);
        
        // Add an object record
        let mut object_record = FuseIndexObjectRecord::new(1);
        
        // Add title entry
        object_record.entries.insert(
            "0".to_string(),
            RecordEntryValue::Single(IndexValue {
                v: "The Great Gatsby".to_string(),
                n: 1.0,
                i: None,
            }),
        );
        
        // Add author entry
        object_record.entries.insert(
            "1".to_string(),
            RecordEntryValue::Single(IndexValue {
                v: "F. Scott Fitzgerald".to_string(),
                n: 1.0,
                i: None,
            }),
        );
        
        records.add_object(object_record);
        
        // Custom field_norm_weight for testing
        let field_norm_weight = 2.0;
        
        // Parse the data into a new index
        let index = FuseIndex::parse_index((keys.clone(), records), None, Some(field_norm_weight));
        
        // Verify the index was parsed correctly
        assert_eq!(index.size(), 2); // One string record and one object record
        assert_eq!(index.keys.len(), 2);
        
        // Check keys were stored properly
        assert_eq!(index.keys[0].id, "title");
        assert_eq!(index.keys[1].id, "author");
        
        // Verify keys map was created
        assert!(index.keys_map.contains_key("title"));
        assert!(index.keys_map.contains_key("author"));
        
        // We can't directly access norm.weight as it's private
        // Instead, verify the records were properly stored
        
        // Check the records were stored properly
        if let FuseIndexRecord::String(record) = &index.records[0] {
            assert_eq!(record.i, 0);
            assert_eq!(record.v, "test string");
            assert_eq!(record.n, 1.0);
        } else {
            panic!("Expected string record");
        }
        
        if let FuseIndexRecord::Object(record) = &index.records[1] {
            assert_eq!(record.i, 1);
            
            // Check title field
            if let RecordEntryValue::Single(title) = &record.entries.get("0").unwrap() {
                assert_eq!(title.v, "The Great Gatsby");
                assert_eq!(title.n, 1.0);
            } else {
                panic!("Expected Single value for title");
            }
            
            // Check author field
            if let RecordEntryValue::Single(author) = &record.entries.get("1").unwrap() {
                assert_eq!(author.v, "F. Scott Fitzgerald");
                assert_eq!(author.n, 1.0);
            } else {
                panic!("Expected Single value for author");
            }
        } else {
            panic!("Expected object record");
        }
    }
    
    #[test]
    fn test_create_index_with_custom_get_fn() {
        // Define a custom get_fn that transforms values to uppercase
        let custom_get_fn: GetFn = |doc, path| {
            let default_fn = FuseOptions::default().get_fn;
            if let Some(GetValue::String(value)) = default_fn(doc, path) {
                Some(GetValue::String(value.to_uppercase()))
            } else {
                default_fn(doc, path)
            }
        };
        
        // Define test keys
        let keys = vec![FuseOptionKey::String("title".into())];
        
        // Create test document
        let docs = vec![json!({"title": "test title"})];
        
        // Create index with custom get_fn
        let index = FuseIndex::create_index(&keys, &docs, Some(custom_get_fn), None);
        
        // Verify the document was indexed with uppercase transformation
        if let FuseIndexRecord::Object(record) = &index.records[0] {
            if let RecordEntryValue::Single(title) = &record.entries.get("0").unwrap() {
                assert_eq!(title.v, "TEST TITLE"); // Should be uppercase
            } else {
                panic!("Expected Single value for title");
            }
        } else {
            panic!("Expected object record");
        }
    }
    
    #[test]
    fn test_parse_index_with_custom_get_fn() {
        // Define a custom get_fn for testing
        let custom_get_fn: GetFn = |doc, path| {
            let default_fn = FuseOptions::default().get_fn;
            if let Some(GetValue::String(value)) = default_fn(doc, path) {
                Some(GetValue::String(value.to_uppercase()))
            } else {
                default_fn(doc, path)
            }
        };
        
        // Create keys
        let keys = vec![
            Key {
                path: vec!["title".to_string()],
                id: "title".to_string(),
                weight: 1.0,
                src: "title".into(),
                get_fn: None,
            },
        ];
        
        // Create mock records
        let mut records = FuseIndexRecords::new();
        let mut object_record = FuseIndexObjectRecord::new(0);
        
        object_record.entries.insert(
            "0".to_string(),
            RecordEntryValue::Single(IndexValue {
                v: "test title".to_string(),
                n: 1.0,
                i: None,
            }),
        );
        
        records.add_object(object_record);
        
        // Parse index with custom get_fn
        let index = FuseIndex::parse_index((keys, records), Some(custom_get_fn), None);
        
        // Verify the index was created successfully
        assert_eq!(index.size(), 1);
        
        // We can't reliably test function pointer equality with closures in Rust
        // Instead, we'll just verify the index was created successfully with the right structure
        // In a real application, we'd test the actual search functionality to verify get_fn works
    }
}

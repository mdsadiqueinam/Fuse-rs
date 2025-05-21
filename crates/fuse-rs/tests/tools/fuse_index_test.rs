use fuse_rs::tools::fuse_index::FuseIndex;
use fuse_rs::core::options::config::FuseOptions;
use fuse_rs::core::options::keys::FuseOptionKey;
use fuse_rs::tools::key_store::Key;
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
    if let fuse_rs::tools::fuse_index_record::FuseIndexRecord::String(record) = &index.records[0] {
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
    if let fuse_rs::tools::fuse_index_record::FuseIndexRecord::Object(record) = &index.records[0] {
        assert_eq!(record.i, 0); // Index should be 0
        assert_eq!(record.entries.len(), 2); // Should have two entries
        
        // Check title field
        if let fuse_rs::tools::fuse_index_record::RecordEntryValue::Single(title_value) = &record.entries.get("0").unwrap() {
            assert_eq!(title_value.v, "The Great Gatsby");
            assert!(title_value.n > 0.0);
        } else {
            panic!("Title should be a Single value");
        }
        
        // Check author field
        if let fuse_rs::tools::fuse_index_record::RecordEntryValue::Single(author_value) = &record.entries.get("1").unwrap() {
            assert_eq!(author_value.v, "F. Scott Fitzgerald");
            assert!(author_value.n > 0.0);
        } else {
            panic!("Author should be a Single value");
        }
    } else {
        panic!("Expected object record");
    }
}
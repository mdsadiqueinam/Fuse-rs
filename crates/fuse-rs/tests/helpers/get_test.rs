use fuse_rs::helpers::get::{get, GetFnPath, GetValue};
use serde_json::json;

/// Sample JSON object for testing
fn test_json() -> serde_json::Value {
    json!({
        "title": "Old Man's War",
        "author": {
            "name": "John Scalzi",
            "age": 18,
            "tags": [
                {
                    "value": "American",
                    "nested": {
                        "value": "nested test 1"
                    }
                },
                {
                    "value": ["sci-fi", "space"],
                    "nested": {
                        "value": "nested test 2"
                    }
                }
            ]
        }
    })
}

#[test]
fn test_get_single_string() {
    let obj = test_json();
    
    // Test array path for simple string value
    let path = GetFnPath::StringArray(vec!["author".into(), "name".into()]);
    let result = get(&obj, &path);
    match result {
        Some(GetValue::String(s)) => assert_eq!(s, "John Scalzi".to_string()),
        _ => panic!("Expected a string"),
    }
}

#[test]
fn test_get_number_as_string() {
    let obj = test_json();
    
    // Test number conversion to string
    let path = GetFnPath::StringArray(vec!["author".into(), "age".into()]);
    let result = get(&obj, &path);
    match result {
        Some(GetValue::String(s)) => assert_eq!(s, "18".to_string()),
        _ => panic!("Expected a string"),
    }
}

#[test]
fn test_get_dot_notation() {
    let obj = test_json();
    
    // Test dot notation path
    let path = GetFnPath::String("author.name".into());
    let result = get(&obj, &path);
    match result {
        Some(GetValue::String(s)) => assert_eq!(s, "John Scalzi".to_string()),
        _ => panic!("Expected a string"),
    }
}

#[test]
fn test_get_array_values() {
    let obj = test_json();
    
    // Test collecting values from arrays
    let path = GetFnPath::StringArray(vec!["author".into(), "tags".into(), "value".into()]);
    let result = get(&obj, &path);
    match result {
        Some(GetValue::Array(arr)) => assert_eq!(arr, vec!["American".to_string(), "sci-fi".to_string(), "space".to_string()]),
        _ => panic!("Expected an array"),
    }
}

#[test]
fn test_get_nested_array_values() {
    let obj = test_json();
    
    // Test collecting deeply nested values from arrays
    let path = GetFnPath::StringArray(vec!["author".into(), "tags".into(), "nested".into(), "value".into()]);
    let result = get(&obj, &path);
    match result {
        Some(GetValue::Array(arr)) => assert_eq!(arr, vec!["nested test 1".to_string(), "nested test 2".to_string()]),
        _ => panic!("Expected an array"),
    }
}
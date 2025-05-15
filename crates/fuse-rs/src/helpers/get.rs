use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

//----------------------------------------------------------------------
// Public API
//----------------------------------------------------------------------

/// Result value returned by path-based JSON object lookup
#[derive(Debug, Clone)]
pub enum GetValue {
    /// A single string value extracted from a JSON object
    String(String),
    /// Multiple string values collected from a JSON array
    Array(Vec<String>),
}

/// Path specification for the get function
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GetFnPath<'a> {
    /// A single string representing a dot-separated path (e.g. "user.address.city")
    String(Cow<'a, str>),
    /// An array of strings representing nested path components (e.g. ["user", "address", "city"])
    StringArray(Vec<Cow<'a, str>>),
}

/// Function type for retrieving values from a JSON object using a path
pub type GetFn = fn(&Value, &GetFnPath) -> Option<GetValue>;

/// Extract values from a JSON object using a path specification
///
/// # Arguments
/// * `obj` - The JSON value to extract data from
/// * `path` - Path specification (either dot notation string or array of path components)
///
/// # Returns
/// * `Some(GetValue::String)` - If a single value was found
/// * `Some(GetValue::Array)` - If multiple values were found (from traversing arrays)
/// * `None` - If the path doesn't exist in the object
pub fn get(obj: &Value, path: &GetFnPath) -> Option<GetValue> {
    match path {
        GetFnPath::String(s) => {
            let path_str: &str = s.as_ref();
            <&str as Get>::get(&path_str, obj)
        }
        GetFnPath::StringArray(arr) => {
            let path_vec: Vec<String> = arr.iter().map(|s| s.to_string()).collect();
            <Vec<String> as Get>::get(&path_vec, obj)
        }
    }
}

/// Default wrapper function for the `get_fn` field
///
/// This returns the default getter function from the `get` module
/// which can access properties by path from a JSON value.
pub fn default_get_fn_wrapper() -> fn(&Value, &GetFnPath) -> Option<GetValue> {
    get
}

//----------------------------------------------------------------------
// Implementation details
//----------------------------------------------------------------------

/// Trait for types that can be used as paths to extract values from JSON objects
pub trait Get {
    /// Extract values from a JSON object
    fn get(&self, obj: &Value) -> Option<GetValue>;
}

/// Implementation for string paths using dot notation (e.g. "user.name")
impl Get for &str {
    fn get(&self, obj: &Value) -> Option<GetValue> {
        let path = self
            .split('.')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        path.get(obj)
    }
}

/// Implementation for array paths (e.g. ["user", "name"])
impl Get for Vec<String> {
    fn get(&self, obj: &Value) -> Option<GetValue> {
        let mut list: Vec<String> = vec![];
        let mut is_array = false;

        get_value(self, obj, &mut list, 0, &mut is_array);

        if list.is_empty() {
            None
        } else if is_array {
            Some(GetValue::Array(list))
        } else {
            Some(GetValue::String(list[0].clone()))
        }
    }
}

/// Helper function to recursively extract values from a JSON object using a path
///
/// This function handles array traversal and value collection.
fn get_value(path: &Vec<String>, obj: &Value, list: &mut Vec<String>, index: usize, is_array: &mut bool) {
    if index >= path.len() {
        match obj {
            Value::String(s) => list.push(s.clone()),
            Value::Bool(b) => list.push(b.to_string()),
            Value::Number(n) => list.push(n.to_string()),
            _ => return,
        }
    } else {
        let key = &path[index];

        // Check if key is a numeric index (for array access)
        let value = if let Ok(num) = key.parse::<usize>() {
            obj.get(num)
        } else {
            obj.get(key)
        };

        match value {
            Some(v) => {
                if v.is_array() {
                    *is_array = true;
                    for item in v.as_array().unwrap() {
                        get_value(path, item, list, index + 1, is_array);
                    }
                } else {
                    get_value(path, v, list, index + 1, is_array);
                }
            },
            None => return,
        }
    }
}

//----------------------------------------------------------------------
// Tests
//----------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::{get, GetFnPath, GetValue};

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
                        "value": "sci-fi",
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
            Some(GetValue::Array(arr)) => assert_eq!(arr, vec!["American".to_string(), "sci-fi".to_string()]),
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
}

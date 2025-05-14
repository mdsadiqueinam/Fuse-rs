use serde_json::Value;

pub enum GetValue {
    String(String),
    Array(Vec<String>),
}

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

        // check if key is number
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

pub trait Get {
    fn get(&self, obj: &Value) -> Option<GetValue>;
}

impl Get for Vec<String> {
    fn get(&self, obj: &Value) -> Option<GetValue> {
        let mut list: Vec<String> = vec![];
        let mut is_array = false;

        get_value(self, obj, &mut list, 0, &mut is_array);

        if is_array {
            Some(GetValue::Array(list))
        } else {
            Some(GetValue::String(list[0].clone()))
        }
    }
}

impl Get for &str {
    fn get(&self, obj: &Value) -> Option<GetValue> {
        let path = self
            .split('.')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        path.get(obj)
    }
}

pub type GetFn<T> = fn(&Value, &T) -> Option<GetValue>;

pub fn get<T: Get>(obj: &Value, path: &T) -> Option<GetValue> {
    path.get(obj)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_get() {
        let obj = json!({
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
        });

        let path = vec!["author".to_string(), "name".to_string()];
        let result = super::get(&obj, &path);
        match result {
            Some(super::GetValue::String(s)) => assert_eq!(s, "John Scalzi".to_string()),
            _ => panic!("Expected a string"),
        }

        let path = vec!["author".to_string(), "age".to_string()];
        let result = super::get(&obj, &path);
        match result {
            Some(super::GetValue::String(s)) => assert_eq!(s, "18".to_string()),
            _ => panic!("Expected a string"),
        }

        let path = "author.name";
        let result = super::get(&obj, &path);
        match result {
            Some(super::GetValue::String(s)) => assert_eq!(s, "John Scalzi".to_string()),
            _ => panic!("Expected a string"),
        }

        let path = vec!["author".to_string(), "tags".to_string(), "value".to_string()];
        let result = super::get(&obj, &path);
        match result {
            Some(super::GetValue::Array(arr)) => assert_eq!(arr, vec!["American".to_string(), "sci-fi".to_string()]),
            _ => panic!("Expected an array"),
        }

        let path = vec!["author".to_string(), "tags".to_string(), "nested".to_string(), "value".to_string()];
        let result = super::get(&obj, &path);
        match result {
            Some(super::GetValue::Array(arr)) => assert_eq!(arr, vec!["nested test 1".to_string(), "nested test 2".to_string()]),
            _ => panic!("Expected an array"),
        }
    }
}

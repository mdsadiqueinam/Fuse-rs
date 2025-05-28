use fuse_rs::core::query_parser::*;
use fuse_rs::FuseOptions;
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn test_is_expression() {
    let mut query = HashMap::new();
    query.insert(LogicalOperator::AND.to_string(), Value::Array(vec![]));
    assert!(is_expression(&query));

    let mut query = HashMap::new();
    query.insert(LogicalOperator::OR.to_string(), Value::Array(vec![]));
    assert!(is_expression(&query));

    let mut query = HashMap::new();
    query.insert("title".to_string(), Value::String("test".to_string()));
    assert!(!is_expression(&query));
}

#[test]
fn test_is_path() {
    let mut query = HashMap::new();
    query.insert(KeyType::PATH.to_string(), Value::Array(vec![]));
    assert!(is_path(&query));

    let mut query = HashMap::new();
    query.insert("title".to_string(), Value::String("test".to_string()));
    assert!(!is_path(&query));
}

#[test]
fn test_is_leaf() {
    let mut obj = serde_json::Map::new();
    obj.insert("title".to_string(), Value::String("test".to_string()));
    assert!(is_leaf(&Value::Object(obj)));

    assert!(!is_leaf(&Value::Array(vec![])));
    assert!(!is_leaf(&Value::String("test".to_string())));
}

#[test]
fn test_convert_to_explicit() {
    let mut query = HashMap::new();
    query.insert("title".to_string(), Value::String("test".to_string()));
    query.insert("author".to_string(), Value::String("john".to_string()));

    let result = convert_to_explicit(&query);
    assert!(result.contains_key(LogicalOperator::AND));
    
    if let Some(Value::Array(arr)) = result.get(LogicalOperator::AND) {
        assert_eq!(arr.len(), 2);
    } else {
        panic!("Expected array for AND operator");
    }
}

#[test]
fn test_parse_simple_query() {
    let options = FuseOptions::default();
    let parse_opts = ParseOptions::default();

    let mut query_map = HashMap::new();
    query_map.insert("title".to_string(), "rust".to_string());
    let query = Expression::KeyValue(query_map);

    let result = parse(query, &options, &parse_opts);
    assert!(result.is_ok());

    if let Ok(ParsedExpression::Logical(node)) = result {
        assert_eq!(node.operator, LogicalOperator::AND);
        assert_eq!(node.children.len(), 1);
    } else {
        panic!("Expected logical node with AND operator");
    }
}

#[test]
fn test_parse_path_query() {
    let options = FuseOptions::default();
    let parse_opts = ParseOptions::default();

    let query = Expression::PathValue {
        path: vec!["author".to_string(), "name".to_string()],
        val: "john".to_string(),
    };

    let result = parse(query, &options, &parse_opts);
    assert!(result.is_ok());

    if let Ok(ParsedExpression::Leaf(leaf)) = result {
        assert!(leaf.key_id.contains("author"));
        assert!(leaf.key_id.contains("name"));
        assert_eq!(leaf.pattern, "john");
    } else {
        panic!("Expected leaf node for path query");
    }
}

#[test]
fn test_parse_and_query() {
    let options = FuseOptions::default();
    let parse_opts = ParseOptions::default();

    let mut title_query = HashMap::new();
    title_query.insert("title".to_string(), "rust".to_string());
    
    let mut author_query = HashMap::new();
    author_query.insert("author".to_string(), "smith".to_string());

    let and_expr = Expression::And {
        and: vec![
            Expression::KeyValue(title_query),
            Expression::KeyValue(author_query),
        ],
    };

    let result = parse(and_expr, &options, &parse_opts);
    assert!(result.is_ok());

    if let Ok(ParsedExpression::Logical(node)) = result {
        assert_eq!(node.operator, LogicalOperator::AND);
        assert_eq!(node.children.len(), 2);
    } else {
        panic!("Expected logical node with AND operator");
    }
}

#[test]
fn test_parse_or_query() {
    let options = FuseOptions::default();
    let parse_opts = ParseOptions::default();

    let or_expr = Expression::Or {
        or: vec![
            Expression::PathValue {
                path: vec!["category".to_string()],
                val: "tech".to_string(),
            },
            Expression::PathValue {
                path: vec!["category".to_string()],
                val: "programming".to_string(),
            },
        ],
    };

    let result = parse(or_expr, &options, &parse_opts);
    assert!(result.is_ok());

    if let Ok(ParsedExpression::Logical(node)) = result {
        assert_eq!(node.operator, LogicalOperator::OR);
        assert_eq!(node.children.len(), 2);
    } else {
        panic!("Expected logical node with OR operator");
    }
}

#[test]
fn test_parse_nested_query() {
    let options = FuseOptions::default();
    let parse_opts = ParseOptions::default();

    // Create a nested query: (title = "rust" AND author = "smith") OR category = "tech"
    let mut title_query = HashMap::new();
    title_query.insert("title".to_string(), "rust".to_string());
    
    let mut author_query = HashMap::new();
    author_query.insert("author".to_string(), "smith".to_string());

    let and_part = Expression::And {
        and: vec![
            Expression::KeyValue(title_query),
            Expression::KeyValue(author_query),
        ],
    };

    let category_part = Expression::PathValue {
        path: vec!["category".to_string()],
        val: "tech".to_string(),
    };

    let nested_query = Expression::Or {
        or: vec![and_part, category_part],
    };

    let result = parse(nested_query, &options, &parse_opts);
    assert!(result.is_ok());

    if let Ok(ParsedExpression::Logical(node)) = result {
        assert_eq!(node.operator, LogicalOperator::OR);
        assert_eq!(node.children.len(), 2);
        
        // First child should be a logical AND node
        if let ParsedExpression::Logical(and_node) = &node.children[0] {
            assert_eq!(and_node.operator, LogicalOperator::AND);
            assert_eq!(and_node.children.len(), 2);
        } else {
            panic!("Expected first child to be a logical AND node");
        }

        // Second child should be a leaf node
        if let ParsedExpression::Leaf(leaf) = &node.children[1] {
            assert_eq!(leaf.pattern, "tech");
        } else {
            panic!("Expected second child to be a leaf node");
        }
    } else {
        panic!("Expected logical node with OR operator");
    }
}

#[test]
fn test_parse_multiple_keys_implicit_and() {
    let options = FuseOptions::default();
    let parse_opts = ParseOptions::default();

    // Multiple keys in one object should be converted to explicit AND
    let mut query_map = HashMap::new();
    query_map.insert("title".to_string(), "rust".to_string());
    query_map.insert("author".to_string(), "smith".to_string());
    let query = Expression::KeyValue(query_map);

    let result = parse(query, &options, &parse_opts);
    assert!(result.is_ok());

    if let Ok(ParsedExpression::Logical(node)) = result {
        assert_eq!(node.operator, LogicalOperator::AND);
        assert_eq!(node.children.len(), 2);
    } else {
        panic!("Expected logical node with implicit AND conversion");
    }
}

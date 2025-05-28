//! Query parser for logical search expressions
//!
//! This module provides functionality to parse and process logical search queries
//! that support AND/OR operations with nested expressions.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::FuseOptions;
use crate::core::error_messages::FuseError;
use crate::tools::key_store::create_key_id;

//----------------------------------------------------------------------
// Constants and Types
//----------------------------------------------------------------------

/// Logical operators for query expressions
pub struct LogicalOperator;

impl LogicalOperator {
    pub const AND: &'static str = "$and";
    pub const OR: &'static str = "$or";
}

/// Key types for path and pattern specification
pub struct KeyType;

impl KeyType {
    pub const PATH: &'static str = "$path";
    pub const PATTERN: &'static str = "$val";
}

/// Represents a search expression that can be parsed and executed
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Expression {
    /// Simple key-value pair expression
    KeyValue(HashMap<String, String>),
    /// Path-based expression with explicit path and value
    PathValue {
        #[serde(rename = "$path")]
        path: Vec<String>,
        #[serde(rename = "$val")]
        val: String,
    },
    /// AND operation with array of sub-expressions
    And {
        #[serde(rename = "$and")]
        and: Vec<Expression>,
    },
    /// OR operation with array of sub-expressions
    Or {
        #[serde(rename = "$or")]
        or: Vec<Expression>,
    },
}

/// Result of parsing a leaf expression (terminal node)
#[derive(Debug, Clone)]
pub struct LeafNode {
    pub key_id: String,
    pub pattern: String,
}

/// Result of parsing a logical expression (intermediate node)
#[derive(Debug, Clone)]
pub struct LogicalNode {
    pub children: Vec<ParsedExpression>,
    pub operator: String,
}

/// The result of parsing any expression
#[derive(Debug, Clone)]
pub enum ParsedExpression {
    Leaf(LeafNode),
    Logical(LogicalNode),
}

/// Options for the parse function
#[derive(Debug, Clone)]
pub struct ParseOptions {
    /// When true, automatically create and add searcher instances
    pub auto: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self { auto: true }
    }
}

//----------------------------------------------------------------------
// Helper Functions
//----------------------------------------------------------------------

/// Check if a query contains logical operators (AND/OR)
pub fn is_expression(query: &HashMap<String, Value>) -> bool {
    query.contains_key(LogicalOperator::AND) || query.contains_key(LogicalOperator::OR)
}

/// Check if a query contains path specification
pub fn is_path(query: &HashMap<String, Value>) -> bool {
    query.contains_key(KeyType::PATH)
}

/// Check if a query is a leaf node (terminal expression)
pub fn is_leaf(query: &Value) -> bool {
    match query {
        Value::Object(obj) => {
            !obj.is_empty() && !is_expression(&obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
        },
        Value::Array(_) => false,
        _ => false,
    }
}

/// Convert implicit query to explicit AND operation
pub fn convert_to_explicit(query: &HashMap<String, Value>) -> HashMap<String, Value> {
    let mut result = HashMap::new();
    let and_expressions: Vec<Value> = query.iter()
        .map(|(key, value)| {
            let mut expr = HashMap::new();
            expr.insert(key.clone(), value.clone());
            Value::Object(serde_json::Map::from_iter(expr.into_iter()))
        })
        .collect();
    
    result.insert(LogicalOperator::AND.to_string(), Value::Array(and_expressions));
    result
}

/// Get string value from JSON Value
fn get_string_value(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        _ => None,
    }
}

//----------------------------------------------------------------------
// Main Parse Function
//----------------------------------------------------------------------

/// Parse a logical search expression into a structured query tree
///
/// # Arguments
///
/// * `query` - The expression to parse
/// * `options` - Fuse search options
/// * `parse_options` - Options controlling parsing behavior
///
/// # Returns
///
/// A `Result` containing the parsed expression tree or an error
///
/// # Example
///
/// ```rust
/// use fuse_rs::core::query_parser::{Expression, parse, ParseOptions};
/// use fuse_rs::FuseOptions;
/// use std::collections::HashMap;
///
/// let options = FuseOptions::default();
/// let parse_opts = ParseOptions::default();
/// 
/// // Simple key-value query
/// let mut query_map = HashMap::new();
/// query_map.insert("title".to_string(), "rust".to_string());
/// let query = Expression::KeyValue(query_map);
/// 
/// let result = parse(query, &options, &parse_opts);
/// ```
pub fn parse(
    query: Expression,
    options: &FuseOptions,
    parse_options: &ParseOptions,
) -> Result<ParsedExpression, FuseError> {
    
    fn parse_next(
        query_value: &Value,
        options: &FuseOptions,
        auto: bool,
    ) -> Result<ParsedExpression, FuseError> {
        if let Value::Object(query_obj) = query_value {
            let keys: Vec<String> = query_obj.keys().cloned().collect();
            let query_map: HashMap<String, Value> = query_obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
            let is_query_path = is_path(&query_map);
            
            // If we have multiple keys and it's not an expression, convert to explicit AND
            if !is_query_path && keys.len() > 1 && !is_expression(&query_map) {
                let explicit_query = convert_to_explicit(&query_map);
                return parse_next(&Value::Object(serde_json::Map::from_iter(explicit_query.into_iter())), options, auto);
            }
            
            // Handle leaf nodes
            if is_leaf(query_value) {
                let (key, pattern) = if is_query_path {
                    // Path-based query
                    let path = query_obj.get(KeyType::PATH)
                        .and_then(|v| v.as_array())
                        .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("$path".to_string()))?;
                    
                    let path_strings: Result<Vec<String>, _> = path.iter()
                        .map(|v| v.as_str().map(|s| s.to_string()).ok_or_else(|| FuseError::InvalidLogicalQueryForKey("$path".to_string())))
                        .collect();
                    let path_strings = path_strings?;
                    
                    let pattern = query_obj.get(KeyType::PATTERN)
                        .and_then(|v| v.as_str())
                        .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("$val".to_string()))?;
                    
                    (create_key_id(&path_strings), pattern.to_string())
                } else {
                    // Simple key-value query
                    let key = keys.get(0)
                        .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("key".to_string()))?;
                    
                    let pattern = get_string_value(query_obj.get(key).unwrap())
                        .ok_or_else(|| FuseError::InvalidLogicalQueryForKey(key.clone()))?;
                    
                    (create_key_id(&[key.clone()]), pattern)
                };
                
                return Ok(ParsedExpression::Leaf(LeafNode {
                    key_id: key,
                    pattern,
                }));
            }
            
            // Handle logical nodes
            let mut node = LogicalNode {
                children: Vec::new(),
                operator: keys.get(0).unwrap_or(&String::new()).clone(),
            };
            
            for key in &keys {
                if let Some(value) = query_obj.get(key) {
                    if let Value::Array(arr) = value {
                        for item in arr {
                            let child = parse_next(item, options, auto)?;
                            node.children.push(child);
                        }
                    }
                }
            }
            
            return Ok(ParsedExpression::Logical(node));
        }
        
        Err(FuseError::InvalidLogicalQueryForKey("query".to_string()))
    }
    
    // Convert expression to Value for processing
    let query_value = match query {
        Expression::KeyValue(map) => {
            let value_map: HashMap<String, Value> = map.into_iter().map(|(k, v)| (k, Value::String(v))).collect();
            if !is_expression(&value_map) {
                let explicit_map = convert_to_explicit(&value_map);
                Value::Object(serde_json::Map::from_iter(explicit_map.into_iter()))
            } else {
                Value::Object(serde_json::Map::from_iter(value_map.into_iter()))
            }
        },
        Expression::PathValue { path, val } => {
            let mut obj = HashMap::new();
            obj.insert(KeyType::PATH.to_string(), Value::Array(path.into_iter().map(Value::String).collect()));
            obj.insert(KeyType::PATTERN.to_string(), Value::String(val));
            Value::Object(serde_json::Map::from_iter(obj.into_iter()))
        },
        Expression::And { and } => {
            let mut obj = HashMap::new();
            let and_values: Result<Vec<Value>, _> = and.into_iter()
                .map(|expr| serde_json::to_value(expr).map_err(|_| FuseError::InvalidLogicalQueryForKey("$and".to_string())))
                .collect();
            obj.insert(LogicalOperator::AND.to_string(), Value::Array(and_values?));
            Value::Object(serde_json::Map::from_iter(obj.into_iter()))
        },
        Expression::Or { or } => {
            let mut obj = HashMap::new();
            let or_values: Result<Vec<Value>, _> = or.into_iter()
                .map(|expr| serde_json::to_value(expr).map_err(|_| FuseError::InvalidLogicalQueryForKey("$or".to_string())))
                .collect();
            obj.insert(LogicalOperator::OR.to_string(), Value::Array(or_values?));
            Value::Object(serde_json::Map::from_iter(obj.into_iter()))
        },
    };
    
    parse_next(&query_value, options, parse_options.auto)
}
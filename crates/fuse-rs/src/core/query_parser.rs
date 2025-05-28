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
// Query Analysis (Single Responsibility)
//----------------------------------------------------------------------

/// Analyzes query structure and type
pub struct QueryAnalyzer;

impl QueryAnalyzer {
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
                let map: HashMap<String, Value> = obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
                !obj.is_empty() && !Self::is_expression(&map)
            },
            Value::Array(_) => false,
            _ => false,
        }
    }

    /// Determine if multiple keys need conversion to explicit AND
    pub fn needs_explicit_and(query: &HashMap<String, Value>) -> bool {
        query.len() > 1 && !Self::is_expression(query) && !Self::is_path(query)
    }
}

//----------------------------------------------------------------------
// Query Transformation (Single Responsibility)
//----------------------------------------------------------------------

/// Handles query transformations and conversions
pub struct QueryTransformer;

impl QueryTransformer {
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

    /// Convert Expression enum to JSON Value for processing
    pub fn expression_to_value(query: Expression) -> Result<Value, FuseError> {
        match query {
            Expression::KeyValue(map) => {
                let value_map: HashMap<String, Value> = map.into_iter()
                    .map(|(k, v)| (k, Value::String(v)))
                    .collect();
                
                // Always convert KeyValue expressions to explicit AND for consistency
                let result = Self::convert_to_explicit(&value_map);
                Ok(Value::Object(serde_json::Map::from_iter(result.into_iter())))
            },
            Expression::PathValue { path, val } => {
                let mut obj = HashMap::new();
                obj.insert(KeyType::PATH.to_string(), 
                    Value::Array(path.into_iter().map(Value::String).collect()));
                obj.insert(KeyType::PATTERN.to_string(), Value::String(val));
                Ok(Value::Object(serde_json::Map::from_iter(obj.into_iter())))
            },
            Expression::And { and } => {
                let and_values: Result<Vec<Value>, _> = and.into_iter()
                    .map(|expr| serde_json::to_value(expr)
                        .map_err(|_| FuseError::InvalidLogicalQueryForKey("$and".to_string())))
                    .collect();
                
                let mut obj = HashMap::new();
                obj.insert(LogicalOperator::AND.to_string(), Value::Array(and_values?));
                Ok(Value::Object(serde_json::Map::from_iter(obj.into_iter())))
            },
            Expression::Or { or } => {
                let or_values: Result<Vec<Value>, _> = or.into_iter()
                    .map(|expr| serde_json::to_value(expr)
                        .map_err(|_| FuseError::InvalidLogicalQueryForKey("$or".to_string())))
                    .collect();
                
                let mut obj = HashMap::new();
                obj.insert(LogicalOperator::OR.to_string(), Value::Array(or_values?));
                Ok(Value::Object(serde_json::Map::from_iter(obj.into_iter())))
            },
        }
    }

    /// Extract string value from JSON Value
    pub fn get_string_value(value: &Value) -> Option<String> {
        value.as_str().map(|s| s.to_string())
    }
}

//----------------------------------------------------------------------
// Node Builders (Single Responsibility)
//----------------------------------------------------------------------

/// Builds leaf nodes from query data
pub struct LeafNodeBuilder;

impl LeafNodeBuilder {
    pub fn build_from_path(query_obj: &serde_json::Map<String, Value>) -> Result<LeafNode, FuseError> {
        let path = query_obj.get(KeyType::PATH)
            .and_then(|v| v.as_array())
            .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("$path".to_string()))?;
        
        let path_strings: Result<Vec<String>, _> = path.iter()
            .map(|v| v.as_str().map(|s| s.to_string())
                .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("$path".to_string())))
            .collect();
        
        let pattern = query_obj.get(KeyType::PATTERN)
            .and_then(|v| v.as_str())
            .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("$val".to_string()))?;
        
        Ok(LeafNode {
            key_id: create_key_id(&path_strings?),
            pattern: pattern.to_string(),
        })
    }

    pub fn build_from_key_value(key: &str, value: &Value) -> Result<LeafNode, FuseError> {
        let pattern = QueryTransformer::get_string_value(value)
            .ok_or_else(|| FuseError::InvalidLogicalQueryForKey(key.to_string()))?;
        
        Ok(LeafNode {
            key_id: create_key_id(&[key.to_string()]),
            pattern,
        })
    }
}

/// Builds logical nodes from query data
pub struct LogicalNodeBuilder;

impl LogicalNodeBuilder {
    pub fn build(operator: &str, query_obj: &serde_json::Map<String, Value>, options: &FuseOptions, auto: bool) -> Result<LogicalNode, FuseError> {
        let mut node = LogicalNode {
            children: Vec::new(),
            operator: operator.to_string(),
        };
        
        if let Some(Value::Array(arr)) = query_obj.get(operator) {
            for item in arr {
                let child = QueryParser::parse_value(item, options, auto)?;
                node.children.push(child);
            }
        }
        
        Ok(node)
    }
}

//----------------------------------------------------------------------
// Legacy helper functions for backward compatibility
//----------------------------------------------------------------------

/// Check if a query contains logical operators (AND/OR)
pub fn is_expression(query: &HashMap<String, Value>) -> bool {
    QueryAnalyzer::is_expression(query)
}

/// Check if a query contains path specification
pub fn is_path(query: &HashMap<String, Value>) -> bool {
    QueryAnalyzer::is_path(query)
}

/// Check if a query is a leaf node (terminal expression)
pub fn is_leaf(query: &Value) -> bool {
    QueryAnalyzer::is_leaf(query)
}

/// Convert implicit query to explicit AND operation
pub fn convert_to_explicit(query: &HashMap<String, Value>) -> HashMap<String, Value> {
    QueryTransformer::convert_to_explicit(query)
}

//----------------------------------------------------------------------
// Main Query Parser (Single Responsibility)
//----------------------------------------------------------------------

/// Main query parser implementing the parsing logic
pub struct QueryParser;

impl QueryParser {
    /// Parse a Value into a ParsedExpression
    pub fn parse_value(
        query_value: &Value,
        options: &FuseOptions,
        auto: bool,
    ) -> Result<ParsedExpression, FuseError> {
        let Value::Object(query_obj) = query_value else {
            return Err(FuseError::InvalidLogicalQueryForKey("query".to_string()));
        };

        let keys: Vec<String> = query_obj.keys().cloned().collect();
        let query_map: HashMap<String, Value> = query_obj.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Handle explicit AND conversion for multiple keys
        if QueryAnalyzer::needs_explicit_and(&query_map) {
            let explicit_query = QueryTransformer::convert_to_explicit(&query_map);
            let explicit_value = Value::Object(serde_json::Map::from_iter(explicit_query.into_iter()));
            return Self::parse_value(&explicit_value, options, auto);
        }

        // Handle leaf nodes
        if QueryAnalyzer::is_leaf(query_value) {
            return Self::parse_leaf_node(query_obj, &keys, &query_map);
        }

        // Handle logical nodes
        Self::parse_logical_node(query_obj, &keys, options, auto)
    }

    fn parse_leaf_node(
        query_obj: &serde_json::Map<String, Value>,
        keys: &[String],
        query_map: &HashMap<String, Value>,
    ) -> Result<ParsedExpression, FuseError> {
        let leaf = if QueryAnalyzer::is_path(query_map) {
            LeafNodeBuilder::build_from_path(query_obj)?
        } else {
            let key = keys.first()
                .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("key".to_string()))?;
            let value = query_obj.get(key).unwrap();
            LeafNodeBuilder::build_from_key_value(key, value)?
        };

        Ok(ParsedExpression::Leaf(leaf))
    }

    fn parse_logical_node(
        query_obj: &serde_json::Map<String, Value>,
        keys: &[String],
        options: &FuseOptions,
        auto: bool,
    ) -> Result<ParsedExpression, FuseError> {
        let operator = keys.first()
            .ok_or_else(|| FuseError::InvalidLogicalQueryForKey("operator".to_string()))?;
        
        let logical_node = LogicalNodeBuilder::build(operator, query_obj, options, auto)?;
        Ok(ParsedExpression::Logical(logical_node))
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
    let query_value = QueryTransformer::expression_to_value(query)?;
    QueryParser::parse_value(&query_value, options, parse_options.auto)
}
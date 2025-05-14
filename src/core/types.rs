use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

/// Represents a nested value for sorted items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseSortItemValue {
    #[serde(rename = "$")]
    pub value: String,
    pub idx: Option<usize>,
}

/// Function item that mirrors TypeScript's FuseSortFunctionItem
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseSortFunctionItem {
    #[serde(flatten)]
    pub fields: HashMap<String, FuseSortItemField>,
}

/// Represents either a single value or an array of values
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuseSortItemField {
    Single(FuseSortItemValue),
    Array(Vec<FuseSortItemValue>),
}

/// Match result type mirroring TypeScript's FuseSortFunctionMatch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseSortFunctionMatch {
    pub score: f64,
    pub key: String,
    pub value: String,
    pub indices: Vec<Vec<usize>>,
}

/// Match list type mirroring TypeScript's FuseSortFunctionMatchList
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseSortFunctionMatchList {
    pub score: f64,
    pub key: String,
    pub value: String,
    pub idx: usize,
    pub indices: Vec<Vec<usize>>,
}

/// Enum to handle both match types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuseSortFunctionMatchType {
    Simple(FuseSortFunctionMatch),
    List(FuseSortFunctionMatchList),
}

/// Argument passed to sort function mirroring TypeScript's FuseSortFunctionArg
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseSortFunctionArg {
    pub idx: usize,
    pub item: FuseSortFunctionItem,
    pub score: f64,
    pub matches: Option<Vec<FuseSortFunctionMatchType>>,
}

/// Function type definition for sort_fn
pub type FuseSortFunction = fn(&FuseSortFunctionArg, &FuseSortFunctionArg) -> i32;

/// Default implementation of the sort function
#[inline]
pub fn default_sort_fn(a: &FuseSortFunctionArg, b: &FuseSortFunctionArg) -> i32 {
    if (a.score - b.score).abs() < f64::EPSILON {
        if a.idx < b.idx { -1 } else { 1 }
    } else {
        if a.score < b.score { -1 } else { 1 }
    }
}

/// Wrapper for default_sort_fn to satisfy Serde's default attribute
pub fn default_sort_fn_wrapper() -> FuseSortFunction {
    default_sort_fn
}

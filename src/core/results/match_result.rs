use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

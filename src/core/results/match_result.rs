//! Types for representing match results in search operations
//!
//! This module contains data structures that represent individual
//! matches found during a search operation, including positional information,
//! matching indices, and score details.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//----------------------------------------------------------------------
// Match Types
//----------------------------------------------------------------------

/// Represents a nested value for sorted items
///
/// This type holds both the actual string value and an optional
/// index to track the original position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseSortItemValue {
    /// The actual string value
    #[serde(rename = "$")]
    pub value: String,
    
    /// Optional index tracking the original position
    pub idx: Option<usize>,
}

/// Function item that mirrors TypeScript's FuseSortFunctionItem
///
/// Contains a collection of fields with their values for sorting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseSortFunctionItem {
    /// Map of field names to their values
    #[serde(flatten)]
    pub fields: HashMap<String, FuseSortItemField>,
}

/// Represents either a single value or an array of values
///
/// This enum allows for flexible representation of field data
/// that could be either singular or multiple.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FuseSortItemField {
    /// A single value for this field
    Single(FuseSortItemValue),
    
    /// Multiple values for this field
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

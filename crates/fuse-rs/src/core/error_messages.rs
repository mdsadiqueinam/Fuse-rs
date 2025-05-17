//! Custom error types used throughout the library
//!
//! This module contains standardized error types that are used by various components
//! of the fuzzy search implementation.

use std::error::Error;
use std::fmt;

/// Custom error enum for fuzzy search operations
#[derive(Debug, Clone)]
pub enum FuseError {
    /// Extended search features are not available
    ExtendedSearchUnavailable,
    
    /// Logical search features are not available
    LogicalSearchUnavailable,
    
    /// Incorrect index type was provided
    IncorrectIndexType,
    
    /// Invalid value for a specific key in a logical search query
    InvalidLogicalQueryForKey(String),
    
    /// Pattern length exceeds the maximum allowed
    PatternLengthTooLarge(usize),
    
    /// A required property is missing in a key
    MissingKeyProperty(String),
    
    /// A key's weight property has an invalid value
    InvalidKeyWeightValue(String),
}

impl fmt::Display for FuseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExtendedSearchUnavailable => write!(f, "Extended search is not available"),
            Self::LogicalSearchUnavailable => write!(f, "Logical search is not available"),
            Self::IncorrectIndexType => write!(f, "Incorrect 'index' type"),
            Self::InvalidLogicalQueryForKey(key) => write!(f, "Invalid value for key {}", key),
            Self::PatternLengthTooLarge(max) => write!(f, "Pattern length exceeds max of {}.", max),
            Self::MissingKeyProperty(name) => write!(f, "Missing {} property in key", name),
            Self::InvalidKeyWeightValue(key) => write!(f, "Property 'weight' in key '{}' must be a positive integer", key),
        }
    }
}

impl Error for FuseError {}

// Legacy function equivalents for backward compatibility
// These can be deprecated in future versions

/// Error message when extended search features are not available
#[deprecated(since = "0.1.0", note = "Use FuseError::ExtendedSearchUnavailable instead")]
pub const EXTENDED_SEARCH_UNAVAILABLE: &str = "Extended search is not available";

/// Error message when logical search features are not available
#[deprecated(since = "0.1.0", note = "Use FuseError::LogicalSearchUnavailable instead")]
pub const LOGICAL_SEARCH_UNAVAILABLE: &str = "Logical search is not available";

/// Error message for an incorrect index type
#[deprecated(since = "0.1.0", note = "Use FuseError::IncorrectIndexType instead")]
pub const INCORRECT_INDEX_TYPE: &str = "Incorrect 'index' type";

/// Error message for an invalid value for a specific key in a logical search query
#[deprecated(since = "0.1.0", note = "Use FuseError::InvalidLogicalQueryForKey instead")]
pub fn LOGICAL_SEARCH_INVALID_QUERY_FOR_KEY(key: &str) -> String {
    format!("Invalid value for key {}", key)
}

/// Error message when pattern length exceeds the maximum allowed
#[deprecated(since = "0.1.0", note = "Use FuseError::PatternLengthTooLarge instead")]
pub fn PATTERN_LENGTH_TOO_LARGE(max: usize) -> String {
    format!("Pattern length exceeds max of {}.", max)
}

/// Error message when a required property is missing in a key
#[deprecated(since = "0.1.0", note = "Use FuseError::MissingKeyProperty instead")]
pub fn MISSING_KEY_PROPERTY(name: &str) -> String {
    format!("Missing {} property in key", name)
}

/// Error message when a key's weight property has an invalid value
#[deprecated(since = "0.1.0", note = "Use FuseError::InvalidKeyWeightValue instead")]
pub fn INVALID_KEY_WEIGHT_VALUE(key: &str) -> String {
    format!("Property 'weight' in key '{}' must be a positive integer", key)
}
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
//! Sorting functionality for search results
//!
//! This module provides types and functions for sorting search results
//! according to relevance scores and other criteria.

use crate::core::results::match_result::FuseSortFunctionArg;

//----------------------------------------------------------------------
// Sort Function Types
//----------------------------------------------------------------------

/// Function type definition for custom sort functions
///
/// This type represents a function that compares two search results
/// and returns their relative ordering.
///
/// Return values:
/// * `-1` if `a` should come before `b`
/// * `1` if `a` should come after `b`
/// * `0` if their order doesn't matter
pub type FuseSortFunction = fn(&FuseSortFunctionArg, &FuseSortFunctionArg) -> i32;

//----------------------------------------------------------------------
// Sort Implementations
//----------------------------------------------------------------------

/// Default implementation of the sort function
///
/// This function sorts results primarily by score (ascending),
/// and then by original index (ascending) when scores are equal.
///
/// # Arguments
///
/// * `a` - First search result to compare
/// * `b` - Second search result to compare
///
/// # Returns
///
/// An integer indicating the relative order: -1 for a before b, 1 for a after b
#[inline]
pub fn default_sort_fn(a: &FuseSortFunctionArg, b: &FuseSortFunctionArg) -> i32 {
    if (a.score - b.score).abs() < f64::EPSILON {
        // When scores are equal, sort by index
        if a.idx < b.idx { -1 } else { 1 }
    } else {
        // Primary sort by score (lower is better)
        if a.score < b.score { -1 } else { 1 }
    }
}

/// Wrapper for default_sort_fn to satisfy Serde's default attribute
///
/// This function exists solely to provide a function pointer
/// to the default sort implementation for use with Serde's
/// default attribute.
pub fn default_sort_fn_wrapper() -> FuseSortFunction {
    default_sort_fn
}

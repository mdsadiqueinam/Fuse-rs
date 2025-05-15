//! Types for representing search results
//!
//! This module contains the primary data structures for representing
//! results returned by the fuzzy search engine.

//----------------------------------------------------------------------
// Search Result Types
//----------------------------------------------------------------------

/// Represents a basic search result item
/// 
/// Contains the matching score and the index of the matched item in the original data array.
/// This is used in the simple search mode when full match details aren't needed.
///
/// # Example
///
/// ```
/// use fuse_rs::SearchResult;
///
/// // A match with the item at index 2 in the original array, with a score of 0.42
/// let result = SearchResult {
///     score: 0.42,
///     idx: 2,
/// };
/// ```
#[derive(Debug, Default)]
pub struct SearchResult {
    /// The calculated relevance score for this match (lower is better)
    pub score: f64,
    
    /// The original index of the matched item in the data array
    pub idx: usize,
}

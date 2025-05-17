//! Types for representing search results
//!
//! This module contains the primary data structures for representing
//! results returned by the fuzzy search engine.

//----------------------------------------------------------------------
// Search Result Types
//----------------------------------------------------------------------

/// Denotes the start/end indices of a match
///
/// # Example
///
/// ```
/// let start_index = 0;
/// let end_index = 5;
///
/// let range: RangeTuple = (start_index, end_index);
/// ```
pub type RangeTuple = (usize, usize);

/// Represents a match within a search result
///
/// Contains information about where the match occurred, including character
/// positions and which key contained the match.
#[derive(Debug, Clone)]
pub struct FuseResultMatch {
    /// Array of index ranges showing where matches occurred
    pub indices: Vec<RangeTuple>,
    
    /// The key in the document where the match was found
    pub key: Option<String>,
    
    /// The reference index of the document in the original collection
    pub ref_index: Option<usize>,
    
    /// The matched value as a string
    pub value: Option<String>,
}

/// Options for controlling search behavior
#[derive(Debug, Clone)]
pub struct FuseSearchOptions {
    /// Maximum number of results to return
    pub limit: usize,
}

/// A complete search result including the matched item and scoring details
///
/// Generic over the item type to allow for different data types in search collections.
#[derive(Debug, Clone)]
pub struct FuseResult<T> {
    /// The original item that matched the search
    pub item: T,
    
    /// The reference index of the matched item in the original collection
    pub ref_index: usize,
    
    /// The relevance score of this match (lower is better)
    pub score: Option<f64>,
    
    /// Details about which parts of the item matched and where
    pub matches: Option<Vec<FuseResultMatch>>,
}

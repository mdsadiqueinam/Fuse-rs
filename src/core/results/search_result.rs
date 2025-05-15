/// Represents a basic search result item
/// 
/// Contains the matching score and the index of the matched item in the original data array.
/// This is used in the simple search mode when full match details aren't needed.
#[derive(Debug, Default)]
pub struct SearchResult {
    /// The calculated relevance score for this match (lower is better)
    pub score: f64,
    /// The original index of the matched item in the data array
    pub idx: usize,
}

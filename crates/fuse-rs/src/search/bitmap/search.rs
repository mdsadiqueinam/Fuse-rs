use std::collections::HashMap;
use crate::FuseOptions;


pub struct SearchResult {
    /// Whether the pattern was found in the text
    pub is_match: bool,
    
    /// The match quality score (lower is better)
    pub score: f64,
    
    /// List of match position ranges as (start, end) tuples
    pub indices: Vec<(usize, usize)>,
}

pub fn search(
    text: &str,
    pattern: &str,
    pattern_alphabet: &HashMap<char, u64>,
    options: &FuseOptions,
) -> Result<SearchResult, PatternTooLongError> {}
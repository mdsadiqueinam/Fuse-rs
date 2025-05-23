
pub struct SearchResult {
    /// Whether the pattern was found in the text
    pub is_match: bool,

    /// The match quality score (lower is better)
    pub score: f64,

    /// List of match position ranges as (start, end) tuples
    pub indices: Option<Vec<(usize, usize)>>,
}

pub trait Searcher {
    fn search_in(&self, text: &str) -> SearchResult;
}
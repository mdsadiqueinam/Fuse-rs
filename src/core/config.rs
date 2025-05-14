use crate::helpers::get;
use serde_json::Value;

/// Configuration options for Fuse.js
pub struct Config {
    // BasicOptions
    /// When `true`, the algorithm continues searching to the end of the input even if a perfect
    /// match is found before the end of the same input.
    pub is_case_sensitive: bool,
    /// When `true`, the algorithm will ignore diacritics (accents) in comparisons
    pub ignore_diacritics: bool,
    /// When true, the search result will include the score
    pub include_score: bool,
    /// List of properties that will be searched. This also supports nested properties.
    pub keys: Vec<String>,
    /// Whether to sort the result list, by score
    pub should_sort: bool,
    /// Sort function for search results
    pub sort_fn: fn(&SearchResult, &SearchResult) -> i8,

    // MatchOptions
    /// Whether the matches should be included in the result set. When `true`, each record in the result
    /// set will include the indices of the matched characters.
    /// These can consequently be used for highlighting purposes.
    pub include_matches: bool,
    /// When `true`, the matching function will continue to the end of a search pattern even if
    /// a perfect match has already been located in the string.
    pub find_all_matches: bool,
    /// Minimum number of characters that must be matched before a result is considered a match
    pub min_match_char_length: usize,

    // FuzzyOptions
    /// Approximately where in the text is the pattern expected to be found?
    pub location: usize,
    /// At what point does the match algorithm give up. A threshold of '0.0' requires a perfect match
    /// (of both letters and location), a threshold of '1.0' would match anything.
    pub threshold: f64,
    /// Determines how close the match must be to the fuzzy location (specified above).
    /// An exact letter match which is 'distance' characters away from the fuzzy location
    /// would score as a complete mismatch. A distance of '0' requires the match be at
    /// the exact location specified; a threshold of '1000' would require a perfect match
    /// to be within 800 characters of the fuzzy location to be found using a 0.8 threshold.
    pub distance: usize,

    // AdvancedOptions
    /// When `true`, it enables the use of unix-like search commands
    pub use_extended_search: bool,
    /// The get function to use when fetching an object's properties.
    /// The default will search nested paths *ie foo.bar.baz*
    pub get_fn: fn(&Value, &Vec<String>) -> Option<get::GetValue>,
    /// When `true`, search will ignore `location` and `distance`, so it won't matter
    /// where in the string the pattern appears.
    pub ignore_location: bool,
    /// When `true`, the calculation for the relevance score (used for sorting) will
    /// ignore the field-length norm.
    pub ignore_field_norm: bool,
    /// The weight to determine how much field length norm effects scoring.
    pub field_norm_weight: usize,
}

/// Represents a search result item
pub struct SearchResult {
    pub score: f64,
    pub idx: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            // BasicOptions defaults
            is_case_sensitive: false,
            ignore_diacritics: false,
            include_score: false,
            keys: Vec::new(),
            should_sort: true,
            sort_fn: |a, b| {
                if a.score == b.score {
                    if a.idx < b.idx { -1 } else { 1 }
                } else {
                    if a.score < b.score { -1 } else { 1 }
                }
            },

            // MatchOptions defaults
            include_matches: false,
            find_all_matches: false,
            min_match_char_length: 1,

            // FuzzyOptions defaults
            location: 0,
            threshold: 0.6,
            distance: 100,

            // AdvancedOptions defaults
            use_extended_search: false,
            get_fn: get::get,
            ignore_location: false,
            ignore_field_norm: false,
            field_norm_weight: 1,
        }
    }
}
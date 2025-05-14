use merge::Merge;
use crate::helpers::get;
use serde_json::Value;
use serde::{Deserialize, Serialize};

/// Default sort function for search results
fn default_sort_fn(a: &SearchResult, b: &SearchResult) -> i8 {
    if a.score == b.score {
        if a.idx < b.idx { -1 } else { 1 }
    } else {
        if a.score < b.score { -1 } else { 1 }
    }
}

/// Wrapper for default_sort_fn to satisfy Serde's default attribute
fn default_sort_fn_wrapper() -> fn(&SearchResult, &SearchResult) -> i8 {
    default_sort_fn
}

/// Wrapper for default_get_fn to satisfy Serde's default attribute
fn default_get_fn_wrapper() -> fn(&Value, &Vec<String>) -> Option<get::GetValue> {
    get::get
}

/// Configuration options for Fuse.js
#[derive(Debug, Clone, Serialize, Deserialize, Merge)]
pub struct Config {
    // BasicOptions
    /// When `true`, the algorithm continues searching to the end of the input even if a perfect
    /// match is found before the end of the same input.
    #[merge(strategy = merge::bool::overwrite_false)]
    pub is_case_sensitive: bool,
    
    /// When `true`, the algorithm will ignore diacritics (accents) in comparisons
    #[merge(strategy = merge::bool::overwrite_false)]
    pub ignore_diacritics: bool,
    
    /// When true, the search result will include the score
    #[merge(strategy = merge::bool::overwrite_false)]
    pub include_score: bool,
   
    /// List of properties that will be searched. This also supports nested properties.
    #[merge(strategy = merge::vec::overwrite_empty)]
    pub keys: Vec<String>,
    
    /// Whether to sort the result list, by score
    #[merge(strategy = merge::bool::overwrite_false)]
    pub should_sort: bool,

    /// Sort function for search results
    #[serde(skip_serializing, skip_deserializing, default = "default_sort_fn_wrapper")]
    #[merge(skip)]
    pub sort_fn: fn(&SearchResult, &SearchResult) -> i8,

    // MatchOptions
    /// Whether the matches should be included in the result set. When `true`, each record in the result
    /// set will include the indices of the matched characters.
    /// These can consequently be used for highlighting purposes.
    #[merge(strategy = merge::bool::overwrite_false)]
    pub include_matches: bool,
    
    /// When `true`, the matching function will continue to the end of a search pattern even if
    /// a perfect match has already been located in the string.
    #[merge(strategy = merge::bool::overwrite_false)]
    pub find_all_matches: bool,
    
    /// Minimum number of characters that must be matched before a result is considered a match
    #[merge(strategy = merge::num::overwrite_zero)]
    pub min_match_char_length: usize,

    // FuzzyOptions
    /// Approximately where in the text is the pattern expected to be found?
    #[merge(strategy = merge::num::overwrite_zero)]
    pub location: usize,
    
    /// At what point does the match algorithm give up. A threshold of '0.0' requires a perfect match
    /// (of both letters and location), a threshold of '1.0' would match anything.
    #[merge(strategy = merge::num::overwrite_zero)]
    pub threshold: f64,
    
    /// Determines how close the match must be to the fuzzy location (specified above).
    /// An exact letter match which is 'distance' characters away from the fuzzy location
    /// would score as a complete mismatch. A distance of '0' requires the match be at
    /// the exact location specified; a threshold of '1000' would require a perfect match
    /// to be within 800 characters of the fuzzy location to be found using a 0.8 threshold.
    #[merge(strategy = merge::num::overwrite_zero)]
    pub distance: usize,

    // AdvancedOptions
    /// When `true`, it enables the use of unix-like search commands
    #[merge(strategy = merge::bool::overwrite_false)]
    pub use_extended_search: bool,

    /// The get function to use when fetching an object's properties.
    /// The default will search nested paths *ie foo.bar.baz*
    #[serde(skip_serializing, skip_deserializing, default = "default_get_fn_wrapper")]
    #[merge(skip)]
    pub get_fn: fn(&Value, &Vec<String>) -> Option<get::GetValue>,
    
    /// When `true`, search will ignore `location` and `distance`, so it won't matter
    /// where in the string the pattern appears.
    #[merge(strategy = merge::bool::overwrite_false)]
    pub ignore_location: bool,
    
    /// When `true`, the calculation for the relevance score (used for sorting) will
    /// ignore the field-length norm.
    #[merge(strategy = merge::bool::overwrite_false)]
    pub ignore_field_norm: bool,
    
    /// The weight to determine how much field length norm effects scoring.
    #[merge(strategy = merge::num::overwrite_zero)]
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
            sort_fn: default_sort_fn,

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
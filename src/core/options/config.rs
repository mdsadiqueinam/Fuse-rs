use crate::core::options::keys::FuseOptionKey;
use crate::core::options::sort::{FuseSortFunction, default_sort_fn, default_sort_fn_wrapper};
use crate::helpers::get::{self, GetFn};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::cmp::max;

/// Default wrapper function for the `get_fn` field
///
/// This returns the default getter function from the `get` module
/// which can access properties by path from a JSON value.
fn default_get_fn_wrapper() -> fn(&Value, &Vec<String>) -> Option<get::GetValue> {
    get::get
}

/// Configuration options for Fuse.js, a powerful fuzzy-search library
/// 
/// `FuseOptions` controls the behavior of the fuzzy search algorithm, allowing
/// fine-grained control over matching behavior, result formatting, and performance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FuseOptions<'a> {
    /// When `true`, the search becomes case-sensitive. Default: `false`
    #[serde(default)]
    pub is_case_sensitive: bool,

    /// When `true`, diacritics (like accents) are ignored in the search. Default: `false`
    #[serde(default)]
    pub ignore_diacritics: bool,
    
    /// When `true`, the score for each match is included in the result. Default: `false`
    #[serde(default)]
    pub include_score: bool,
    
    /// The keys (properties) in the items to search. This can be:
    /// - A single string key
    /// - An array of string keys
    /// - A key object with name and weight
    /// Default: empty array
    #[serde(default)]
    pub keys: Vec<FuseOptionKey<'a>>,
    
    /// When `true`, the matching results will be sorted by score. Default: `true`
    #[serde(default)]
    pub should_sort: bool,

    /// Function used to sort the results. Takes two search result arguments.
    /// Default: sort by score ascending, then by index ascending
    #[serde(skip, default = "default_sort_fn_wrapper")]
    pub sort_fn: FuseSortFunction,

    /// When `true`, the matching character positions are included in results. Default: `false`
    #[serde(default)]
    pub include_matches: bool,
    
    /// When `true`, all matches are found, not just the first match per item. Default: `false`
    #[serde(default)]
    pub find_all_matches: bool,
    
    /// Minimum number of characters that must be matched before a result is considered. Default: `1`
    #[serde(default)]
    pub min_match_char_length: usize,

    /// Determines approximately where in the text the pattern is expected to be found. Default: `0`
    #[serde(default)]
    pub location: usize,
    
    /// At what point does the match algorithm give up. A threshold of `0.0` requires a perfect match.
    /// A threshold of `1.0` matches anything. Default: `0.6`
    #[serde(default)]
    pub threshold: f64,
    
    /// Determines how close the match must be to the fuzzy location. Default: `100`
    /// An exact letter match which is `distance` characters away from the fuzzy location
    /// would score as a complete mismatch.
    #[serde(default)]
    pub distance: usize,

    /// When `true`, enables the extended search mode which allows for more flexibility. Default: `false`
    #[serde(default)]
    pub use_extended_search: bool,

    /// Function used to retrieve a value from an item for comparison.
    /// Default: Basic property accessor function
    #[serde(skip, default = "default_get_fn_wrapper")]
    pub get_fn: GetFn<Vec<String>>,

    /// When `true`, search will ignore `location` and `distance`. Default: `false`
    #[serde(default)]
    pub ignore_location: bool,
    
    /// When `true`, similarity scoring is disabled and field length normalization is ignored. Default: `false`
    #[serde(default)]
    pub ignore_field_norm: bool,
    
    /// Determines the importance of field length normalization. Default: `1`
    #[serde(default)]
    pub field_norm_weight: usize,
}

impl<'a> Default for FuseOptions<'a> {
    fn default() -> Self {
        Self {
            is_case_sensitive: false,
            ignore_diacritics: false,
            include_score: false,
            keys: Vec::new(),
            should_sort: true,
            sort_fn: default_sort_fn,
            include_matches: false,
            find_all_matches: false,
            min_match_char_length: 1,
            location: 0,
            threshold: 0.6,
            distance: 100,
            use_extended_search: false,
            get_fn: get::get,
            ignore_location: false,
            ignore_field_norm: false,
            field_norm_weight: 1,
        }
    }
}

impl<'a> FuseOptions<'a> {
    /// Create a new `FuseOptions` instance with default values
    pub fn new() -> Self {
        Default::default()
    }

    /// Validates and normalizes the options
    ///
    /// This ensures that options are within valid ranges and consistent with each other.
    pub fn validate(&mut self) -> &mut Self {
        // Ensure threshold is between 0.0 and 1.0
        self.threshold = self.threshold.max(0.0).min(1.0);
        
        // Ensure min_match_char_length is at least 1
        self.min_match_char_length = max(self.min_match_char_length, 1);
        
        // Ensure distance is at least 0
        self.distance = max(self.distance, 0);
        
        // Ensure field_norm_weight is at least 1
        self.field_norm_weight = max(self.field_norm_weight, 1);
        
        self
    }

    /// Get a validated copy of the options
    pub fn validated(&self) -> Self {
        let mut opts = self.clone();
        opts.validate();
        opts
    }
}

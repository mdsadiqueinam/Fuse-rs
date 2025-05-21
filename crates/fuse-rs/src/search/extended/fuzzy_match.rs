use lazy_static::lazy_static;
use regex::Regex;

use super::base_match::BaseMatch;
use crate::search::bitmap::create_pattern_alphabet::create_pattern_alphabet;
use crate::search::bitmap::search as bitmap_search;
use crate::search::search_result::SearchResult;
use crate::FuseOptions;

/// Options for fuzzy matching that don't require a lifetime parameter
#[derive(Clone)]
pub struct FuzzyMatchOptions {
    pub location: usize,
    pub threshold: f64,
    pub distance: usize,
    pub include_matches: bool,
    pub find_all_matches: bool,
    pub min_match_char_length: usize,
    pub ignore_location: bool,
    pub ignore_field_norm: bool,
    pub field_norm_weight: f64,
}

impl From<&FuseOptions<'_>> for FuzzyMatchOptions {
    fn from(options: &FuseOptions<'_>) -> Self {
        Self {
            location: options.location,
            threshold: options.threshold,
            distance: options.distance,
            include_matches: options.include_matches,
            find_all_matches: options.find_all_matches,
            min_match_char_length: options.min_match_char_length,
            ignore_location: options.ignore_location,
            ignore_field_norm: options.ignore_field_norm,
            field_norm_weight: options.field_norm_weight,
        }
    }
}

impl From<FuzzyMatchOptions> for FuseOptions<'static> {
    fn from(options: FuzzyMatchOptions) -> Self {
        let mut fuse_options = FuseOptions::default();
        fuse_options.location = options.location;
        fuse_options.threshold = options.threshold;
        fuse_options.distance = options.distance;
        fuse_options.include_matches = options.include_matches;
        fuse_options.find_all_matches = options.find_all_matches;
        fuse_options.min_match_char_length = options.min_match_char_length;
        fuse_options.ignore_location = options.ignore_location;
        fuse_options.ignore_field_norm = options.ignore_field_norm;
        fuse_options.field_norm_weight = options.field_norm_weight;
        fuse_options
    }
}

/// Fuzzy match implementation
/// Match type: fuzzy-match
/// Description: Items that fuzzy match the pattern
pub struct FuzzyMatch {
    pattern: String,
    options: FuzzyMatchOptions,
}

impl FuzzyMatch {
    /// Create a new FuzzyMatch
    pub fn new(pattern: String, options: FuseOptions<'_>) -> Self {
        Self { 
            pattern, 
            options: FuzzyMatchOptions::from(&options),
        }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "fuzzy"
    }
}

impl BaseMatch for FuzzyMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        lazy_static! {
            static ref MULTI_REGEX: Regex = Regex::new(r#"^"(.*)"$"#).unwrap();
        }
        &MULTI_REGEX
    }

    fn single_regex() -> &'static Regex {
        lazy_static! {
            static ref SINGLE_REGEX: Regex = Regex::new(r"^(.*)$").unwrap();
        }
        &SINGLE_REGEX
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        // Create pattern alphabet for bitmap search
        let pattern_alphabet = create_pattern_alphabet(&self.pattern);

        // Convert FuzzyMatchOptions to FuseOptions
        let fuse_options: FuseOptions = self.options.clone().into();

        // Perform bitmap search
        let bitmap_result = bitmap_search::search(text, &self.pattern, &pattern_alphabet, &fuse_options)
            .unwrap_or_else(|_| SearchResult {
                is_match: false,
                score: 1.0,
                indices: None,
            });

        // Convert bitmap search result to our SearchResult
        SearchResult {
            is_match: bitmap_result.is_match,
            score: bitmap_result.score,
            indices: bitmap_result.indices,
        }
    }
}

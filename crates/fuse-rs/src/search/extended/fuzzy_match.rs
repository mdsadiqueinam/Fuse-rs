use lazy_static::lazy_static;
use regex::Regex;

use crate::FuseOptions;
use crate::search::bitmap::search as bitmap_search;
use crate::helpers::str_ext::StrExt;
use crate::search::bitmap::create_pattern_alphabet::create_pattern_alphabet;
use crate::search::search_result::SearchResult;
use super::base_match::{BaseMatch};

/// Fuzzy match implementation
/// Match type: fuzzy-match
/// Description: Items that fuzzy match the pattern
pub struct FuzzyMatch<'a> {
    pattern: String,
    options: FuseOptions<'a>,
}

impl<'a> FuzzyMatch<'a> {
    /// Create a new FuzzyMatch
    pub fn new(pattern: String, options: FuseOptions<'a>) -> Self {
        Self { pattern, options }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "fuzzy"
    }
}

impl<'a> BaseMatch for FuzzyMatch<'a> {
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

        // Perform bitmap search
        let bitmap_result = bitmap_search::search(text, &self.pattern, &pattern_alphabet, &self.options)
            .unwrap_or_else(|_| crate::search::search_result::SearchResult {
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

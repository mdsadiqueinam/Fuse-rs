use lazy_static::lazy_static;
use regex::Regex;
use std::borrow::Cow;

use super::base_match::BaseMatch;
use crate::search::bitmap::bitmap_search::BitmapSearch;
use crate::search::search::{SearchResult, Searcher};
use crate::FuseOptions;

/// Fuzzy match implementation
/// Match type: fuzzy-match
/// Description: Items that fuzzy match the pattern
pub struct FuzzyMatch<'a> {
    pattern: String,
    options: Cow<'a, FuseOptions<'a>>,
    bitmap_search: BitmapSearch<'a>
}

impl<'a> FuzzyMatch<'a> {
    /// Create a new FuzzyMatch
    pub fn new(pattern: String, options: Cow<'a, FuseOptions<'a>>) -> Self {
        let bitmap_search = BitmapSearch::new(Cow::Owned(pattern.clone()), options.clone());
        
        Self { 
            pattern, 
            options,
            bitmap_search
        }
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
        self.bitmap_search.search_in(text)
    }
    
}

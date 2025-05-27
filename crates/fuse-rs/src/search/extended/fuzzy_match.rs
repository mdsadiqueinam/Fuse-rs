use regex::Regex;
use std::borrow::Cow;
use std::sync::OnceLock;
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
    bitmap_search: BitmapSearch<'a>,
}

impl<'a> FuzzyMatch<'a> {
    /// Create a new FuzzyMatch
    pub fn new(pattern: String, options: Cow<'a, FuseOptions<'a>>) -> Self {
        let bitmap_search = BitmapSearch::new(Cow::Owned(pattern.clone()), options.clone());
        
        Self { 
            pattern, 
            options,
            bitmap_search,
        }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "fuzzy"
    }
}

static MULTI_REGEX: OnceLock<Regex> = OnceLock::new();

static SINGLE_REGEX: OnceLock<Regex> = OnceLock::new();

impl<'a> BaseMatch for FuzzyMatch<'a> {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        let multi_regex = MULTI_REGEX.get_or_init(|| {
            Regex::new(r#"^"(.*)"$"#).unwrap() // Example regex, adjust as needed
        });
        &multi_regex
    }

    fn single_regex() -> &'static Regex {
        let single_regex = SINGLE_REGEX.get_or_init(|| {
            Regex::new(r"^(.*)$").unwrap() // Example regex, adjust as needed
        });
        &single_regex
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        self.bitmap_search.search_in(text)
    }
    
}

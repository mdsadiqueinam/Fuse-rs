use lazy_static::lazy_static;
use regex::Regex;
use crate::search::search_result::SearchResult;
use super::base_match::{BaseMatch};

/// Inverse suffix exact match implementation
/// Token: '!file$'
/// Match type: inverse-suffix-exact-match
/// Description: Items that do not end with `file`
pub struct InverseSuffixExactMatch {
    pattern: String,
}

impl InverseSuffixExactMatch {
    /// Create a new InverseSuffixExactMatch
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "inverse-suffix-exact"
    }
}

impl BaseMatch for InverseSuffixExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        lazy_static! {
            static ref MULTI_REGEX: Regex = Regex::new(r#"^!"(.*)"\$$"#).unwrap();
        }
        &MULTI_REGEX
    }

    fn single_regex() -> &'static Regex {
        lazy_static! {
            static ref SINGLE_REGEX: Regex = Regex::new(r"^!(.*)\$$").unwrap();
        }
        &SINGLE_REGEX
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        let is_match = !text.ends_with(&self.pattern);

        SearchResult {
            is_match,
            score: if is_match { 0.0 } else { 1.0 },
            indices: if is_match {
                Some(vec![(0, text.len().saturating_sub(1))])
            } else {
                None
            },
        }
    }
}

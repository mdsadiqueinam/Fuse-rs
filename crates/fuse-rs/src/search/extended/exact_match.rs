use lazy_static::lazy_static;
use regex::Regex;
use crate::search::search_result::SearchResult;
use super::base_match::{BaseMatch};

/// Exact match implementation
/// Token: '=file'
/// Match type: exact-match
/// Description: Items that are `file`
pub struct ExactMatch {
    pattern: String,
}

impl ExactMatch {
    /// Create a new ExactMatch
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    /// Get the match type as a static method
    pub fn get_type() -> &'static str {
        "exact"
    }
}

impl BaseMatch for ExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        lazy_static! {
            static ref MULTI_REGEX: Regex = Regex::new(r#"^="(.*)"$"#).unwrap();
        }
        &MULTI_REGEX
    }

    fn single_regex() -> &'static Regex {
        lazy_static! {
            static ref SINGLE_REGEX: Regex = Regex::new(r"^=(.*)$").unwrap();
        }
        &SINGLE_REGEX
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }
}

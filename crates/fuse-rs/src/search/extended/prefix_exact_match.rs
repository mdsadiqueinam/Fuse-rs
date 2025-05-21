use lazy_static::lazy_static;
use regex::Regex;
use crate::search::search_result::SearchResult;
use super::base_match::{BaseMatch};

/// Prefix exact match implementation
/// Token: '^file'
/// Match type: prefix-exact-match
/// Description: Items that start with `file`
pub struct PrefixExactMatch {
    pattern: String,
}

impl PrefixExactMatch {
    /// Create a new PrefixExactMatch
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "prefix-exact"
    }
}

impl BaseMatch for PrefixExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        lazy_static! {
            static ref MULTI_REGEX: Regex = Regex::new(r#"^\^"(.*)"$"#).unwrap();
        }
        &MULTI_REGEX
    }

    fn single_regex() -> &'static Regex {
        lazy_static! {
            static ref SINGLE_REGEX: Regex = Regex::new(r"^\^(.*)$").unwrap();
        }
        &SINGLE_REGEX
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }
}

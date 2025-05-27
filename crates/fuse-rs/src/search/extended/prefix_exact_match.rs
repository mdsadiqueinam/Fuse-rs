use regex::Regex;
use std::sync::OnceLock;
use crate::search::search::SearchResult;
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

static MULTI_REGEX: OnceLock<Regex> = OnceLock::new();
static SINGLE_REGEX: OnceLock<Regex> = OnceLock::new();

impl BaseMatch for PrefixExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        MULTI_REGEX.get_or_init(|| Regex::new(r#"^\^"(.*)"$"#).unwrap())
    }

    fn single_regex() -> &'static Regex {
        SINGLE_REGEX.get_or_init(|| Regex::new(r"^\^(.*)$").unwrap())
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        let is_match = text.starts_with(self.pattern());

        SearchResult {
            is_match,
            score: if is_match { 0.0 } else { 1.0 },
            indices: if is_match {
                Some(vec![(0, self.pattern().len().saturating_sub(1))])
            } else {
                None
            },
        }
    }
}

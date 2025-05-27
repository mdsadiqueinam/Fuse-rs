use regex::Regex;
use std::sync::OnceLock;
use crate::search::search::SearchResult;
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

static MULTI_REGEX: OnceLock<Regex> = OnceLock::new();
static SINGLE_REGEX: OnceLock<Regex> = OnceLock::new();

impl BaseMatch for ExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        MULTI_REGEX.get_or_init(|| Regex::new(r#"^="(.*)"$"#).unwrap())
    }

    fn single_regex() -> &'static Regex {
        SINGLE_REGEX.get_or_init(|| Regex::new(r"^=(.*)$").unwrap())
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        let is_match = text == self.pattern();

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

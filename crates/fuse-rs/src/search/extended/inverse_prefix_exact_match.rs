use regex::Regex;
use std::sync::OnceLock;
use crate::search::search::SearchResult;
use super::base_match::{BaseMatch};

/// Inverse prefix exact match implementation
/// Token: '!^file'
/// Match type: inverse-prefix-exact-match
/// Description: Items that do not start with `file`
pub struct InversePrefixExactMatch {
    pattern: String,
}

impl InversePrefixExactMatch {
    /// Create a new InversePrefixExactMatch
    pub fn new(pattern: String) -> Self {
        Self { pattern }
    }

    /// Get the match type
    pub fn get_type() -> &'static str {
        "inverse-prefix-exact"
    }
}

static MULTI_REGEX: OnceLock<Regex> = OnceLock::new();
static SINGLE_REGEX: OnceLock<Regex> = OnceLock::new();

impl BaseMatch for InversePrefixExactMatch {
    fn pattern(&self) -> &str {
        &self.pattern
    }

    fn multi_regex() -> &'static Regex {
        MULTI_REGEX.get_or_init(|| Regex::new(r#"^!\^"(.*)"$"#).unwrap())
    }

    fn single_regex() -> &'static Regex {
        SINGLE_REGEX.get_or_init(|| Regex::new(r"^!\^(.*)$").unwrap())
    }

    fn get_type(&self) -> &'static str {
        Self::get_type()
    }

    fn search(&self, text: &str) -> SearchResult {
        let is_match = !text.starts_with(&self.pattern);

        SearchResult {
            is_match,
            score: if is_match { 0.0 } else { 1.0 },
            indices: Some(vec![(0, text.len().saturating_sub(1))]),
        }
    }
}
